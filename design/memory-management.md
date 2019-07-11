# Artichoke Ruby Memory Management

Artichoke has no garbage collector and relies on
[Rust's built-in memory management](https://pcwalton.github.io/2013/03/18/an-overview-of-memory-management-in-rust.html)
and a [cycle-aware reference-counted smart pointer](/cactusref) of its own
invention to reclaim memory when Ruby [`Value`](value.md)s are no longer
reachable from the VM.

This document refers to data structures with backticks if it is refering to a
specific implementation, for example, [`Value`](value.md). If the data structure
is not formatted as code, the document is referring to the general concept, for
example, HashMap does not refer to
[`HashMap`](https://doc.rust-lang.org/std/collections/struct.HashMap.html), but
rather the concept of a hash table.

## `BasicObject#object_id`

`BasicObject` is the root of the class hierarchy in Ruby. All
[`Value`](value.md)s inherit from `BasicObject`. Every `BasicObject` must have a
unique `object_id`, which is a `u64`. There are some wrinkles to this, but for
now we can assume that every `Value` that the VM allocates will have a unique
`object_id`.

In the VM, `object_id` is represented by the following struct:

```rust
#[derive(Clone, Debug)]
pub struct ObjectId {
    // reference to the VM
    interp: Artichoke,
    // opaque and immutable identifier
    id: usize,
}

impl ObjectId {
    pub fn id(&self) -> usize {
        self.id
    }
}

impl Hash for ObjectId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl PartialEq for ObjectId {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for ObjectId {}
```

`ObjectId` is useful as a pointer. By having a reference to an `ObjectId`,
components of the VM can retrieve `Value`s from the heap.

Mediating access to the underlying `Value`s via the `ObjectId` allows us to
centrally implement guards around mutability. For example, `Value`s can be
marked immutable with
[`Object#freeze`](https://ruby-doc.org/core-2.6.3/Object.html#method-i-freeze).
`ObjectId` implements
[`Deref`](https://doc.rust-lang.org/std/ops/trait.Deref.html) and
[`DerefMut`](https://doc.rust-lang.org/std/ops/trait.DerefMut.html) that resolve
a `Value` on the heap via its `ObjectId` and enforces mutability guarantees of
the VM.

## The Heap

The heap is a HashMap from `ObjectId` to a sharable `Value` representation. The
_shareable `Value` representation_ is a wrapper around `Value` that enables it
to have shared ownership. The specifics of the wrapper depend on VM context (for
example, values are wrapped differently if they are shared by multiple threads),
but conceptually the wrapper behaves like an `Rc<RefCell<Value>>`. The wrapper
can have multiple owners, supports weak references, and allows the `Value` to be
mutated.

The heap stores weak references to `Value`. When a `Value` takes an owned
reference to another, it resolves the value wrapper via the heap and upgrades
the weak reference into a strong reference.

Eventually, a `Value` may become unreachable, the strong count on the `Rc` that
wraps it will drop to zero, the memory will be reclaimed, and the weak reference
becomes invalid. To optimize access times for the heap and prevent the heap from
growing unbounded, a background thread will periodically compact the heap by
removing `ObjectId`s that point to invalid weak references.

## Shared References and Reference Counting

A `Value` can be referenced by many other `Value`s. For example, in the below
program, the String `'artichoke'` is reachable from six locations.

```ruby
x = 'artichoke'
# name binding
y = x
# collection item
ary = [x, x]
# captured variable
f = proc { x }
# self-referential structure
x.instance_variable_set :@a, x
```

Because instance variables are publically settable, every `Value` can hold a
reference other `Value`s, including cyclical ones. This means we cannot ignore
cycles.

`CactusRef` is a smart pointer that behaves similarly to `Rc`, with the addition
that `CactusRef` can detect cycles and deallocate `Value`s if they form an
orphaned cycle.

When a special `Value` takes ownership of another, the `ObjectId` of the other
value resolves a strong `CactusRef` via the heap and stores it in its internal
data structures (e.g. an instance variable table or a `Vec` backing an Array).

For example, an Array is backed by a `Vec<CactusRef<RefCell<Value>>>` and the
symbol table of instance variables on an object is a
`HashMap<Identifier, CactusRef<RefCell<Value>>>`.

When a `CactusRef` is dropped, the reference count of the `Value` decreases and
`CactusRef` does a reachability check using breadth-first search and the
`Reachable` implementation on `Value`.

Consider the following code:

```ruby
class Container
  attr_accessor :inner

  def initialize(inner)
    @inner = inner
  end
end

def make_cycle
  a = Container.new(nil) # ObjectId(100)
  b = Container.new(a)   # ObjectId(200)
  c = Container.new(b)   # ObjectId(300)
  d = Container.new(c)   # ObjectId(400)
  a.inner = d
  a
end

ring = make_cycle
```

Here's what happens from the perspective of the VM:

1. The `a` binding holds a strong reference to the Value `ObjectId(100)`
2. `ObjectId(200)` holds a strong reference to the Value `ObjectId(100)`
3. The `b` binding holds a strong reference to the Value `ObjectId(200)`
4. `ObjectId(300)` holds a strong reference to the Value `ObjectId(200)`
5. The `c` binding holds a strong reference to the Value `ObjectId(300)`
6. `ObjectId(400)` holds a strong reference to the Value `ObjectId(300)`
7. The `d` binding holds a strong reference to the Value `ObjectId(400)`

At this point the strong counts look like this:

| `ObjectId` | Strong Count |
| ---------- | ------------ |
| 100        | 2            |
| 200        | 2            |
| 300        | 2            |
| 400        | 1            |

Assigning `ObjectId(400)` to the `@inner` instance variable of `ObjectId(100)`
makes these four `Value`s form a cycle.

### Detecting Cycles

Each `Value` can answer the question: Can I reach an `ObjectId`?

```rust
unsafe impl Reachable for Value {
    pub fn object_id(&self) -> usize {
      self.object_id.id()
    }

    pub fn can_reach(&self, object_id: usize) -> bool {
        for value in self.instance_variables {
            if value.object_id.id() == object_id {
                return true;
            }
        }
        // and for other data structures like Class, Array, Hash
        unimplemented!();
    }
}
```

`Value` does not need to do a full graph traversal because `CactusRef` does it.

Back to our example: when `ObjectId(400)` is assigned to `@inner` on
`ObjectId(100)`, the VM detects a cycle because `ObjectId(100)` is reachable by
the chain of `ObjectId(400) -> ObjectId(300) -> ObjectId(200) -> ObjectId(100)`.
The Valuse associated with the `ObjectId`s are reachable in these ways:

| `ObjectId` | Binding              |
| ---------- | -------------------- |
| 100        | `a`, `ObjectId(200)` |
| 200        | `b`, `ObjectId(300)` |
| 300        | `c`, `ObjectId(400)` |
| 400        | `d`, `ObjectId(100)` |

Once we return from the function, the variable bindings get dropped:

| `ObjectId` | Binding                 |
| ---------- | ----------------------- |
| 100        | `ring`, `ObjectId(200)` |
| 200        | `ObjectId(300)`         |
| 300        | `ObjectId(400)`         |
| 400        | `ObjectId(100)`         |

When `ring` is dropped or reassigned, `CactusRef` detects an orphaned cycle and
will deallocate all of the `Value`s.

# frozen_string_literal: true

def spec
  empty_get
  inline_get
  dynamic_get

  empty_slice
  inline_slice
  dynamic_slice

  inline_set
  inline_set_sparse
  dynamic_set
  dynamic_set_sparse

  inline_set_with_drain
  dynamic_set_with_drain

  true
end

def empty_get
  a = []
  raise unless a[0].nil?
  raise unless a[100].nil?
  raise unless a[-1].nil?
  raise unless a[-100].nil?
end

def inline_get
  a = [1, 2, 3]
  raise unless a[0] == 1
  raise unless a[100].nil?
  raise unless a[-1] == 3
  raise unless a[-100].nil?
end

def dynamic_get
  a = (1..25).map.to_a
  raise unless a[0] == 1
  raise unless a[10] == 11
  raise unless a[100].nil?
  raise unless a[-1] == 25
  raise unless a[-100].nil?
end

def empty_slice
  a = []
  raise unless a[0, 0] == []
  raise unless a[1, 10] == []
end

def inline_slice
  a = [1, 2, 3]
  raise unless a[0, 0] == []
  raise unless a[1, 10] == [2, 3]
  raise unless a[10, 5] == []
end

def dynamic_slice
  a = (1..25).map.to_a
  raise unless a[0, 0] == []
  raise unless a[1, 10] == [2, 3, 4, 5, 6, 7, 8, 9, 10, 11]
  raise unless a[10, 5] == [11, 12, 13, 14, 15]
  raise unless a[22, 10] == [23, 24, 25]
  raise unless a[100, 10] == []
end

def inline_set
  a = [1, 2, 3]
  a[0] = 'a'
  raise unless a == ['a', 2, 3]

  a = [1, 2, 3]
  a[1] = 'a'
  raise unless a == [1, 'a', 3]

  a = [1, 2, 3]
  a[-1] = 'a'
  raise unless a == [1, 2, 'a']

  a = [1, 2, 3]
  a[3] = 'a'
  raise unless a == [1, 2, 3, 'a']
end

def inline_set_sparse
  # to inline
  a = [1, 2, 3]
  a[6] = 'a'
  raise unless a == [1, 2, 3, nil, nil, nil, 'a']

  # to dynamic
  a = [1, 2, 3]
  a[20] = 'a'
  raise unless a == [1, 2, 3, nil, nil, nil, nil, nil, nil, nil, nil, nil, nil, nil, nil, nil, nil, nil, nil, nil, 'a']
end

def dynamic_set
  a = (1..25).map.to_a
  a[0] = 'a'
  raise unless a == ['a', 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25]

  a = (1..25).map.to_a
  a[10] = 'a'
  raise unless a == [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 'a', 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25]

  a = (1..25).map.to_a
  a[-1] = 'a'
  raise unless a == [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 'a']

  a = (1..25).map.to_a
  a[25] = 'a'
  raise unless a == [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 'a']
end

def dynamic_set_sparse
  a = (1..25).map.to_a
  a[30] = 'a'
  raise unless a == [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, nil, nil, nil, nil, nil, 'a']
end

def inline_set_with_drain
  a = [1, 2, 3]
  a[0, 0] = 'a'
  raise unless a == ['a', 1, 2, 3]

  a = [1, 2, 3]
  a[1, 0] = 'a'
  raise unless a == [1, 'a', 2, 3]

  a = [1, 2, 3]
  a[3, 0] = 'a'
  raise unless a == [1, 2, 3, 'a']

  a = [1, 2, 3]
  a[6, 0] = 'a'
  raise unless a == [1, 2, 3, nil, nil, nil, 'a']

  a = [1, 2, 3]
  a[6, 10] = 'a'
  raise unless a == [1, 2, 3, nil, nil, nil, 'a']

  a = [1, 2, 3]
  a[0, 100] = 'a'
  raise unless a == ['a']

  a = [1, 2, 3]
  a[1, 1] = 'a'
  raise unless a == [1, 'a', 3]

  a = [1, 2, 3]
  a[1, 2] = 'a'
  raise unless a == [1, 'a']

  a = [1, 2, 3]
  a[1, 10] = 'a'
  raise unless a == [1, 'a']

  a = [1, 2, 3]
  a[3, 2] = 'a'
  raise unless a == [1, 2, 3, 'a']

  a = [1, 2, 3, 4, 5, 6, 7, 8]
  a[7, 100] = 'a'
  raise unless a == [1, 2, 3, 4, 5, 6, 7, 'a']
end

def dynamic_set_with_drain
  a = (1..25).map.to_a
  a[0, 0] = 'a'
  raise unless a == ['a', 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25]

  a = (1..25).map.to_a
  a[1, 0] = 'a'
  raise unless a == [1, 'a', 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25]

  a = (1..25).map.to_a
  a[25, 0] = 'a'
  raise unless a == [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 'a']

  a = (1..25).map.to_a
  a[27, 0] = 'a'
  raise unless a == [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, nil, nil, 'a']

  a = (1..25).map.to_a
  a[27, 10] = 'a'
  raise unless a == [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, nil, nil, 'a']

  a = (1..25).map.to_a
  a[0, 100] = 'a'
  raise unless a == ['a']

  a = (1..25).map.to_a
  a[1, 1] = 'a'
  raise unless a == [1, 'a', 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25]

  a = (1..25).map.to_a
  a[1, 2] = 'a'
  raise unless a == [1, 'a', 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25]

  a = (1..25).map.to_a
  a[1, 24] = 'a'
  raise unless a == [1, 'a']

  a = (1..25).map.to_a
  a[1, 100] = 'a'
  raise unless a == [1, 'a']

  a = (1..25).map.to_a
  a[20, 100] = 'a'
  raise unless a == [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 'a']

  a = (1..25).map.to_a
  a[25, 100] = 'a'
  raise unless a == [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 'a']
end

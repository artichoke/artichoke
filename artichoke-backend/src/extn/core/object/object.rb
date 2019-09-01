class Object
  def itself
    self
  end

  def tap
    yield self
    self
  end

  def yield_self(&block)
    return block.call(self) if block

    to_enum :yield_self
  end

  alias then yield_self
end

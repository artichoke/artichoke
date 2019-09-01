class Object
  def tap
    yield self
    self
  end

  def yield_self(&block)
    return to_enum :yield_self unless block

    block.call(self)
  end

  alias then yield_self
end

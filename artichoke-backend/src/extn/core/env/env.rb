# frozen_string_literal: true

class EnvClass
    def delete(name)
        value = self[name]
        self[name] = nil
        value
    end
end

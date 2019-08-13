# frozen_string_literal: true

class EnvClass
    def assoc(name)
        value = self[name]

        return value.nil? ? nil :  [name, self[name]]
    end

    def clear
        self.to_h.each do |var_name, var_value|
            if ! var_value.nil?
                self[var_name] = nil
            end
        end

        self.to_h
    end

    def delete(name)
        value = self[name]
        self[name] = nil

        if block_given? and value.nil?
            yield name
        end

        value
    end

    def empty?
        return self.to_h.empty?
    end

    def has_key?(name)
        ! self[name].nil?
    end

    def has_value?(value)
        ! self.key(value).nil?
    end

    def include?(name)
        self.has_key?(name)
    end

    def key(value)
        result = nil

        self.to_h.each do |var_name, var_value|
            if var_value == value
                result = var_name
                break
            end
        end

        result
    end

    def length
        self.to_h.length
    end

    def size
        self.length
    end

    def keys
        self.to_h.map {|var_name, var_value| var_name}
    end

    def rehash
        nil
    end

    def to_a
        self.to_h.map {|var_name, var_value| [var_name, var_value]}
    end

    def to_s
        'ENV'
    end

    def value?(name)
        ! self.key(name).nil?
    end

    def values
        self.to_h.map {|var_name, var_value| var_value}
    end

    def slice(*keys)
        self.to_h.slice(*keys)
    end

end

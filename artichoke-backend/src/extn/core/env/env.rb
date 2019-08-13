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
        self.to_h.has_key?(name)
    end

    def has_value?(value)
        self.to_h.has_value?(value)
    end

    def include?(name)
        self.to_h.has_key?(name)
    end

    def key(value)
        self.to_h.key(value)
    end

    def length
        self.to_h.length
    end

    def size
        self.to_h.size
    end

    def keys
        self.to_h.keys
    end

    def rehash
        nil
    end

    def to_a
        self.to_h.to_a
    end

    def to_s
        'ENV'
    end

    def value?(name)
        self.to_h.value?(name)
    end

    def values
        self.to_h.values
    end

    def slice(*keys)
        self.to_h.slice(*keys)
    end

    def values_at(*names)
        self.to_h.values_at(*names)
    end

    def to_hash
        self.to_h
    end

    def shift 
        envs = self.to_h
        
        a_pair = envs.shift

        if a_pair.nil?
            return nil
        else
            self[a_pair[0]] = nil
            return a_pair
        end
    end

    def update(hash)
        hash.each do |key, value|
            self[key] = value
        end

        self.to_h
    end
end

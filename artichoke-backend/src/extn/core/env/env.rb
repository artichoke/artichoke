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

end

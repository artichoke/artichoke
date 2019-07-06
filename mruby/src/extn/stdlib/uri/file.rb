# frozen_string_literal: true

require_relative 'generic'

module URI
  #
  # The "file" URI is defined by RFC8089.
  #
  class File < Generic
    # A Default port of nil for URI::File.
    DEFAULT_PORT = nil

    #
    # An Array of the available components for URI::File.
    #
    COMPONENT = %i[
      scheme
      host
      path
    ].freeze

    #
    # == Description
    #
    # Creates a new URI::File object from components, with syntax checking.
    #
    # The components accepted are +host+ and +path+.
    #
    # The components should be provided either as an Array, or as a Hash
    # with keys formed by preceding the component names with a colon.
    #
    # If an Array is used, the components must be passed in the
    # order <code>[host, path]</code>.
    #
    # Examples:
    #
    #     require 'uri'
    #
    #     uri1 = URI::File.build(['host.example.com', '/path/file.zip'])
    #     uri1.to_s  # => "file://host.example.com/path/file.zip"
    #
    #     uri2 = URI::File.build({:host => 'host.example.com',
    #       :path => '/ruby/src'})
    #     uri2.to_s  # => "file://host.example.com/ruby/src"
    #
    def self.build(args)
      tmp = Util.make_components_hash(self, args)
      super(tmp)
    end

    # Protected setter for the host component +v+.
    #
    # See also URI::Generic.host=.
    #
    def set_host(host) # rubocop:disable Naming/AccessorMethodName
      host = '' if host.nil? || host == 'localhost'
      @host = host
    end

    # do nothing
    def set_port(port); end # rubocop:disable Naming/AccessorMethodName

    # raise InvalidURIError
    def check_userinfo(_user)
      raise URI::InvalidURIError, 'can not set userinfo for file URI'
    end

    # raise InvalidURIError
    def check_user(_user)
      raise URI::InvalidURIError, 'can not set user for file URI'
    end

    # raise InvalidURIError
    def check_password(_user)
      raise URI::InvalidURIError, 'can not set password for file URI'
    end

    # do nothing
    def set_userinfo(userinfo); end # rubocop:disable Naming/AccessorMethodName

    # do nothing
    def set_user(user); end # rubocop:disable Naming/AccessorMethodName

    # do nothing
    def set_password(password); end # rubocop:disable Naming/AccessorMethodName
  end

  @@schemes['FILE'] = File
end

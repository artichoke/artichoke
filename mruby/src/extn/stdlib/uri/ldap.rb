# frozen_string_literal: false

# = uri/ldap.rb
#
# Author::
#  Takaaki Tateishi <ttate@jaist.ac.jp>
#  Akira Yamada <akira@ruby-lang.org>
# License::
#   URI::LDAP is copyrighted free software by Takaaki Tateishi and Akira Yamada.
#   You can redistribute it and/or modify it under the same term as Ruby.
# Revision:: $Id$
#
# See URI for general documentation
#

require_relative 'generic'

module URI
  #
  # LDAP URI SCHEMA (described in RFC2255).
  #--
  # ldap://<host>/<dn>[?<attrs>[?<scope>[?<filter>[?<extensions>]]]]
  #++
  class LDAP < Generic
    # A Default port of 389 for URI::LDAP.
    DEFAULT_PORT = 389

    # An Array of the available components for URI::LDAP.
    COMPONENT = %i[
      scheme
      host port
      dn
      attributes
      scope
      filter
      extensions
    ].freeze

    # Scopes available for the starting point.
    #
    # * SCOPE_BASE - the Base DN
    # * SCOPE_ONE  - one level under the Base DN, not including the base DN and
    #   not including any entries under this
    # * SCOPE_SUB  - subtrees, all entries at all levels
    #
    SCOPE = [
      SCOPE_ONE = 'one'.freeze,
      SCOPE_SUB = 'sub'.freeze,
      SCOPE_BASE = 'base'.freeze
    ].freeze

    #
    # == Description
    #
    # Creates a new URI::LDAP object from components, with syntax checking.
    #
    # The components accepted are host, port, dn, attributes,
    # scope, filter, and extensions.
    #
    # The components should be provided either as an Array, or as a Hash
    # with keys formed by preceding the component names with a colon.
    #
    # If an Array is used, the components must be passed in the
    # order <code>[host, port, dn, attributes, scope, filter, extensions]</code>.
    #
    # Example:
    #
    #     uri = URI::LDAP.build({:host => 'ldap.example.com',
    #       :dn => '/dc=example'})
    #
    #     uri = URI::LDAP.build(["ldap.example.com", nil,
    #       "/dc=example;dc=com", "query", nil, nil, nil])
    #
    def self.build(args)
      tmp = Util.make_components_hash(self, args)

      tmp[:path] = tmp[:dn] if tmp[:dn]

      query = []
      %i[extensions filter scope attributes].collect do |x|
        next if !tmp[x] && query.empty?

        query.unshift(tmp[x])
      end

      tmp[:query] = query.join('?')

      super(tmp)
    end

    #
    # == Description
    #
    # Creates a new URI::LDAP object from generic URI components as per
    # RFC 2396. No LDAP-specific syntax checking is performed.
    #
    # Arguments are +scheme+, +userinfo+, +host+, +port+, +registry+, +path+,
    # +opaque+, +query+, and +fragment+, in that order.
    #
    # Example:
    #
    #     uri = URI::LDAP.new("ldap", nil, "ldap.example.com", nil, nil,
    #       "/dc=example;dc=com", nil, "query", nil)
    #
    # See also URI::Generic.new.
    #
    def initialize(*arg)
      super(*arg)

      raise InvalidURIError, 'bad LDAP URL' if @fragment

      parse_dn
      parse_query
    end

    # Private method to cleanup +dn+ from using the +path+ component attribute.
    def parse_dn
      @dn = @path[1..-1]
    end
    private :parse_dn

    # Private method to cleanup +attributes+, +scope+, +filter+, and +extensions+
    # from using the +query+ component attribute.
    def parse_query
      @attributes = nil
      @scope      = nil
      @filter     = nil
      @extensions = nil

      if @query
        attrs, scope, filter, extensions = @query.split('?')

        @attributes = attrs if attrs && !attrs.empty?
        @scope      = scope if scope && !scope.empty?
        @filter     = filter if filter && !filter.empty?
        @extensions = extensions if extensions && !extensions.empty?
      end
    end
    private :parse_query

    # Private method to assemble +query+ from +attributes+, +scope+, +filter+, and +extensions+.
    def build_path_query
      @path = '/' + @dn

      query = []
      [@extensions, @filter, @scope, @attributes].each do |x|
        next if !x && query.empty?

        query.unshift(x)
      end
      @query = query.join('?')
    end
    private :build_path_query

    # Returns dn.
    attr_reader :dn

    # Private setter for dn +val+.
    def set_dn(val)
      @dn = val
      build_path_query
      @dn
    end
    protected :set_dn

    # Setter for dn +val+.
    def dn=(val)
      set_dn(val)
      val
    end

    # Returns attributes.
    attr_reader :attributes

    # Private setter for attributes +val+.
    def set_attributes(val)
      @attributes = val
      build_path_query
      @attributes
    end
    protected :set_attributes

    # Setter for attributes +val+.
    def attributes=(val)
      set_attributes(val)
      val
    end

    # Returns scope.
    attr_reader :scope

    # Private setter for scope +val+.
    def set_scope(val)
      @scope = val
      build_path_query
      @scope
    end
    protected :set_scope

    # Setter for scope +val+.
    def scope=(val)
      set_scope(val)
      val
    end

    # Returns filter.
    attr_reader :filter

    # Private setter for filter +val+.
    def set_filter(val)
      @filter = val
      build_path_query
      @filter
    end
    protected :set_filter

    # Setter for filter +val+.
    def filter=(val)
      set_filter(val)
      val
    end

    # Returns extensions.
    attr_reader :extensions

    # Private setter for extensions +val+.
    def set_extensions(val)
      @extensions = val
      build_path_query
      @extensions
    end
    protected :set_extensions

    # Setter for extensions +val+.
    def extensions=(val)
      set_extensions(val)
      val
    end

    # Checks if URI has a path.
    # For URI::LDAP this will return +false+.
    def hierarchical?
      false
    end
  end

  @@schemes['LDAP'] = LDAP
end

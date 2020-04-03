# frozen_string_literal: true

require 'json'

def spec
  parse_json
  generate_json
  convert_obj_to_json

  true
end

def parse_json
  my_hash = JSON.parse('{"hello": "goodbye"}')
  raise unless my_hash['hello'] == 'goodbye'
end

def generate_json
  my_hash = { hello: 'goodbye' }
  raise unless JSON.generate(my_hash) == '{"hello":"goodbye"}'
  raise unless { hello: 'goodbye' }.to_json == '{"hello":"goodbye"}'
end

def convert_obj_to_json
  raise unless 1.to_json == '1'
end

spec if $PROGRAM_NAME == __FILE__

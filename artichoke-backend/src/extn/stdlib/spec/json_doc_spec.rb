# frozen_string_literal: true

require 'json'

describe JSON do
  it 'parses JSON' do
    my_hash = JSON.parse('{"hello": "goodbye"}')
    my_hash['hello'].should eql('goodbye')
  end

  it 'generates JSON' do
    my_hash = { hello: 'goodbye' }
    JSON.generate(my_hash).should eql('{"hello":"goodbye"}')
    { hello: 'goodbye' }.to_json.should eql('{"hello":"goodbye"}')
  end

  it 'converts Ruby objects to JSON' do
    1.to_json.should eql('1')
  end
end

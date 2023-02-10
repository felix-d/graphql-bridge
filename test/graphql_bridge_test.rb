# frozen_string_literal: true

require 'json'
require "test/unit"
require_relative "../lib/graphql_bridge"


class Schema
  class SomeObject
    def some_other_field
      "some other value"
    end
  end

  def initialize
    @some_object = SomeObject.new
  end

  def some_field
    "some value"
  end

  def some_object
    @some_object
  end
end

class GraphQLBridgeTest < Test::Unit::TestCase
  def test_it_works
    bridge = GraphQLBridge.new(Schema.new)
    result = bridge.eval(<<~GQL)
      {
        someField
        someObject {
          someOtherField
        }
      }
    GQL

    expected = {
      "someField" => "some value",
      "someObject" => {
        "someOtherField" => "some other value"
      }
    }.to_json

    assert_equal(expected, result)
  end
end

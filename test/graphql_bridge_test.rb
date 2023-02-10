# frozen_string_literal: true

require "test/unit"
require_relative "../lib/graphql_bridge"


class Schema
  def some_field
    puts "resolving some field"
  end
end

class GraphQLBridgeTest < Test::Unit::TestCase
  def test_it_works
    bridge = GraphQLBridge.new(Schema.new)
    assert(bridge.execute(<<~GQL))
      {
        someField
      }
    GQL
  end
end

# frozen_string_literal: true

require "test/unit"
require_relative "../lib/graphql_bridge"

class GraphQLBridgeTest < Test::Unit::TestCase
  def test_it_works
    assert(parse_fixture("query"))
  end

  private

  def parse_fixture(name)
    GraphQLBridge.parse(File.read(File.join(__dir__, "fixtures/#{name}.gql")))
  end
end

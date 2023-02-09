Gem::Specification.new do |s|
  s.name = "graphql_bridge"
  s.version = "0.1.0"
  s.summary = "GraphQL bridge"
  s.authors = ["Felix Descoteaux"]
  s.extensions = ["ext/graphql_bridge/extconf.rb"]

  # needed until rubygems supports Rust support is out of beta
  s.add_dependency "rb_sys", "~> 0.9.39"

  # only needed when developing or packaging your gem
  s.add_development_dependency "rake-compiler", "~> 1.2.0"
end

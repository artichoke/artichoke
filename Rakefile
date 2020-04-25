# frozen_string_literal: true

require 'fileutils'

task default: 'lint:all'

namespace :lint do
  desc 'Lint and format'
  task all: %i[format clippy rubocop eslint]

  desc 'Run clippy'
  task :clippy do
    roots = Dir.glob('**/{lib,main}.rs')
    roots.each do |root|
      FileUtils.touch(root)
    end
    sh 'cargo clippy'
  end

  desc 'Run rubocop'
  task :rubocop do
    sh 'rubocop -a'
  end

  desc 'Format sources'
  task format: :deps do
    sh 'cargo fmt -- --color=auto'
    sh 'npm run fmt:all'
    sh 'node scripts/clang-format.js'
  end

  desc 'Run eslint'
  task eslint: :deps do
    sh 'npx eslint --fix .'
  end

  desc 'Check markdown links'
  task :links do
    markdown = [
      'BUILD.md',
      'CONTRIBUTING.md',
      'README.md',
      'RUBYSPEC.md',
      'VISION.md',
      'artichoke-backend/README.md',
      'artichoke-backend/src/extn/stdlib/gen/README.md',
      'artichoke-backend/vendor/README.md',
      'artichoke-core/README.md',
      'spec-runner/README.md',
      'spec-runner/vendor/README.md'
    ]
    markdown.each do |source|
      sh "npx markdown-link-check --config .github/markdown-link-check.json #{source}"
    end
  end

  desc 'Install linting dependencies'
  task :deps do
    sh 'npm install'
  end

  desc 'Lint with restriction pass (unenforced lints)'
  task :restriction do
    sh 'cargo clippy -- ' \
      '-W clippy::dbg_macro ' \
      '-W clippy::get_unwrap ' \
      '-W clippy::indexing_slicing ' \
      '-W clippy::option_expect_used ' \
      '-W clippy::option_unwrap_used ' \
      '-W clippy::panic ' \
      '-W clippy::print_stdout ' \
      '-W clippy::result_expect_used ' \
      '-W clippy::result_unwrap_used ' \
      '-W clippy::todo ' \
      '-W clippy::unimplemented ' \
      '-W clippy::unreachable'
  end
end

desc 'Generate Rust API documentation'
task :doc do
  sh 'rustup run --install nightly cargo doc --workspace --no-deps'
end

desc 'Generate Rust API documentation and open it in a web browser'
task :'doc:open' do
  sh 'rustup run --install nightly cargo doc --workspace --no-deps --open'
end

desc 'Run enforced ruby/spec suite'
task :spec do
  sh 'cargo run -q -p spec-runner -- spec-runner/enforced-specs.yaml'
end

desc 'Run Artichoke Rust tests'
task :test do
  sh 'cargo test --workspace'
end

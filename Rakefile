# frozen_string_literal: true

require 'fileutils'

task default: :lint

desc 'Lint and format'
task lint: %i[lint:format lint:clippy lint:rubocop]

namespace :lint do
  desc 'Run Clippy'
  task :clippy do
    roots = Dir.glob('**/{build,lib,main}.rs')
    roots.each do |root|
      FileUtils.touch(root)
    end
    sh 'cargo clippy --workspace --all-features'
  end

  desc 'Run RuboCop'
  task :rubocop do
    sh 'rubocop -a'
  end

  desc 'Format sources'
  task :format do
    sh 'cargo fmt -- --color=auto'
    sh "npx prettier --write '**/*'"
    sh 'npx github:artichoke/clang-format'
  end

  desc 'Format sources (alias)'
  task fmt: :format

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

  desc 'Lint with Clippy restriction pass (unenforced lints)'
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

desc 'Build Rust workspace'
task :build do
  sh 'cargo build --workspace'
end

desc 'Generate Rust API documentation'
task :doc do
  ENV['RUSTFLAGS'] = '-D warnings'
  ENV['RUSTDOCFLAGS'] = '-D warnings --cfg docsrs'
  sh 'rustup run --install nightly cargo doc --workspace'
end

desc 'Generate Rust API documentation and open it in a web browser'
task :'doc:open' do
  ENV['RUSTFLAGS'] = '-D warnings'
  ENV['RUSTDOCFLAGS'] = '-D warnings --cfg docsrs'
  sh 'rustup run --install nightly cargo doc --workspace --open'
end

desc 'Run enforced ruby/spec suite'
task :spec do
  sh 'cargo run -q -p spec-runner -- spec-runner/enforced-specs.yaml'
end

desc 'Run Artichoke unit tests'
task :test do
  sh 'cargo test --workspace'
end

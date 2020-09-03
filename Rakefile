# frozen_string_literal: true

require 'fileutils'
require 'shellwords'
require 'rubocop/rake_task'

task default: %i[format lint]

desc 'Lint sources'
task lint: %i[lint:clippy lint:rubocop:auto_correct]

namespace :lint do
  desc 'Lint Rust sources with Clippy'
  task :clippy do
    roots = Dir.glob('**/{build,lib,main}.rs')
    roots.each do |root|
      FileUtils.touch(root)
    end
    sh 'cargo clippy --workspace --all-features'
  end

  RuboCop::RakeTask.new(:rubocop)

  desc 'Check for broken markdown links'
  task :links do
    readmes = Dir.glob('**/README.md')
      .reject { |path| path.include?('node_modules/') }
      .reject { |path| path.include?('target/') }
      .reject { |path| path.include?('vendor/') }
    docs = Dir.glob('*.md')
    extra = [
      'artichoke-backend/src/extn/stdlib/vendor/gen/README.md',
      'artichoke-backend/vendor/README.md',
      'spec-runner/vendor/README.md'
    ]
    markdown = readmes + docs + extra
    puts 'Checking links in the following markdown sources:', markdown.sort
    markdown.sort.uniq.each do |source|
      sh "npx markdown-link-check --config .github/markdown-link-check.json #{source}"
      sleep(rand(1..5))
    end
  end

  desc 'Lint Rust sources with Clippy restriction pass (unenforced lints)'
  task :'clippy:restriction' do
    roots = Dir.glob('**/{build,lib,main}.rs')
    roots.each do |root|
      FileUtils.touch(root)
    end
    lints = [
      'clippy::dbg_macro',
      'clippy::get_unwrap',
      'clippy::indexing_slicing',
      'clippy::panic',
      'clippy::print_stdout',
      'clippy::expect_used',
      'clippy::unwrap_used',
      'clippy::todo',
      'clippy::unimplemented',
      'clippy::unreachable'
    ]
    command = ['cargo', 'clippy', '--'] + lints.flat_map { |lint| ['-W', lint] }
    sh command.shelljoin
  end
end

desc 'Format sources'
task format: %i[format:rust format:text format:c]

namespace :format do
  desc 'Format Rust sources with rustfmt'
  task :rust do
    sh 'cargo fmt -- --color=auto'
  end

  desc 'Format text, YAML, and Markdown sources with prettier'
  task :text do
    sh "npx prettier --write '**/*'"
  end

  desc 'Format .c and .h sources with clang-format'
  task :c do
    sh 'npx github:artichoke/clang-format'
  end
end

desc 'Format sources'
task fmt: %i[fmt:rust fmt:text fmt:c]

namespace :fmt do
  desc 'Format Rust sources with rustfmt'
  task :rust do
    sh 'cargo fmt -- --color=auto'
  end

  desc 'Format text, YAML, and Markdown sources with prettier'
  task :text do
    sh "npx prettier --write '**/*'"
  end

  desc 'Format .c and .h sources with clang-format'
  task :c do
    sh 'npx github:artichoke/clang-format'
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

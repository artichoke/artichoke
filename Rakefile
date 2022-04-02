# frozen_string_literal: true

require 'open-uri'
require 'shellwords'
require 'bundler/audit/task'
require 'rubocop/rake_task'

task default: %i[format lint]

desc 'Lint sources'
task lint: %i[lint:clippy lint:rubocop:auto_correct]

namespace :lint do
  RuboCop::RakeTask.new(:rubocop)

  desc 'Lint Rust sources with Clippy'
  task :clippy do
    sh 'cargo clippy --workspace --all-features --all-targets'
    Dir.chdir('spec-runner') do
      sh 'cargo clippy --workspace --all-features --all-targets'
    end
    Dir.chdir('ui-tests') do
      sh 'cargo clippy --workspace --all-features --all-targets'
    end
  end

  desc 'Lint Rust sources with Clippy restriction pass (unenforced lints)'
  task :'clippy:restriction' do
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
    sh 'rustup run --install nightly cargo fmt -- --color=auto'
    Dir.chdir('spec-runner') do
      sh 'rustup run --install nightly cargo fmt -- --color=auto'
    end
    Dir.chdir('ui-tests') do
      sh 'rustup run --install nightly cargo fmt -- --color=auto'
    end
  end

  desc 'Format text, YAML, and Markdown sources with prettier'
  task :text do
    sh 'npx prettier --write "**/*"'
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
    sh 'rustup run --install nightly cargo fmt -- --color=auto'
    Dir.chdir('spec-runner') do
      sh 'rustup run --install nightly cargo fmt -- --color=auto'
    end
    Dir.chdir('ui-tests') do
      sh 'rustup run --install nightly cargo fmt -- --color=auto'
    end
  end

  desc 'Format text, YAML, and Markdown sources with prettier'
  task :text do
    sh 'npx prettier --write "**/*"'
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

desc 'Build Rust workspace and sub-workspaces'
task :'build:all' do
  sh 'cargo build --workspace'
  Dir.chdir('fuzz') do
    sh 'cargo build --workspace'
  end
  Dir.chdir('spec-runner') do
    sh 'cargo build --workspace'
  end
  Dir.chdir('ui-tests') do
    sh 'cargo build --workspace'
  end
end

desc 'Generate Rust API documentation'
task :doc do
  ENV['RUSTDOCFLAGS'] = '-D warnings -D rustdoc::broken_intra_doc_links --cfg docsrs'
  sh 'rustup run --install nightly cargo doc --workspace'
end

desc 'Generate Rust API documentation and open it in a web browser'
task :'doc:open' do
  ENV['RUSTDOCFLAGS'] = '-D warnings -D rustdoc::broken_intra_doc_links --cfg docsrs'
  sh 'rustup run --install nightly cargo doc --workspace --open'
end

desc 'Run enforced ruby/spec suite'
task :spec do
  Dir.chdir('spec-runner') do
    sh 'cargo run -q -- enforced-specs.toml'
  end
end

desc 'Run Artichoke unit tests'
task test: %i[test:unit]

namespace :test do
  # TODO: Add fuzz into all list when tests work
  desc 'Run all tests'
  task all: %i[unit ui]

  desc 'Run fuzz tests (Fuzz the interpreter for crashes with arbitrary input)'
  task :fuzz do
    Dir.chdir('fuzz') do
      sh 'cargo test --workspace'
    end
  end

  desc 'Run ui tests (check exact stdout/stderr of Artichoke binaries)'
  task :ui do
    sh 'cargo build'
    Dir.chdir('ui-tests') do
      sh 'cargo test --workspace'
    end
  end

  desc 'Run unit tests'
  task :unit do
    sh 'cargo test --workspace'
  end
end

desc 'Run Artichoke with LeakSanitizer'
task :'sanitizer:leak' do
  ENV['RUSTFLAGS'] = '-Z sanitizer=leak'
  ENV['RUST_BACKTRACE'] = '1'
  host = `rustc -vV | grep host | cut -d' ' -f2`.chomp
  command = ['rustup', 'run', '--install', 'nightly', 'cargo', 'test', '--workspace', '--all-features', '--target', host]
  sh command.shelljoin
end

Bundler::Audit::Task.new

namespace :release do
  link_check_files = FileList.new('**/*.md') do |f|
    f.exclude('node_modules/**/*')
    f.exclude('**/target/**/*')
    f.exclude('**/vendor/**/*')
    f.include('*.md')
    f.include('**/vendor/*.md')
  end

  link_check_files.uniq.sort.each do |markdown|
    desc 'Check for broken links in markdown files'
    task markdown_link_check: markdown do
      command = ['npx', 'markdown-link-check', '--config', '.github/markdown-link-check.json', markdown]
      sh command.shelljoin
      sleep(rand(1..5))
    end
  end
end

namespace :toolchain do
  desc 'Sync Rust toolchain to all sources'
  task sync: %i[sync:manifests sync:ci]

  rust_toolchain = File.read('rust-toolchain').chomp

  namespace :sync do
    desc 'Sync the root rust-toolchain version to all crate manifests'
    task :manifests do
      rust_version = File.read('rust-toolchain').chomp
      regexp = /^rust-version = "(.*)"$/
      next_rust_version = "rust-version = \"#{rust_toolchain}\""

      pkg_files = FileList.new('*/Cargo.toml').include('Cargo.toml')

      failures = pkg_files.map do |file|
        contents = File.read(file)

        if (existing_version = contents.match(regexp))
          File.write(file, contents.gsub(regexp, next_rust_version)) if existing_version != rust_version
          next
        end

        puts "Failed to update #{file}, ensure there is a rust-version specified" if Rake.verbose
        file
      end.compact

      raise 'Failed to update some rust-versions' if failures.any?
    end

    desc 'Sync the root rust-toolchain version to CI jobs'
    task :ci do
      workflow_files = FileList.new('.github/workflows/*.yaml')

      workflow_files.each do |file|
        contents = File.read(file)

        File.write(file, contents.gsub(/(rustup toolchain install) \d+\.\d+\.\d+/, "\\1 #{rust_toolchain}"))
      end
    end
  end
end

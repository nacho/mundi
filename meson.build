project('mundi', 'rust',
  version: '0.4.0',
  meson_version: '>= 0.59.0',
)

i18n = import('i18n')
gnome = import('gnome')

cargo = find_program('cargo')
rustc = find_program('rustc')

gtk4_dep = dependency('gtk4', version: '>= 4.14')
adwaita_dep = dependency('libadwaita-1', version: '>= 1.5')

gettext_package = meson.project_name()
localedir = get_option('prefix') / get_option('localedir')
datadir = get_option('prefix') / get_option('datadir')
application_id = 'io.github.nacho.mundi'

conf = configuration_data()
conf.set('LOCALEDIR', localedir)
conf.set('DATADIR', datadir)
conf.set('GETTEXT_PACKAGE', gettext_package)
conf.set('VERSION', meson.project_version())
conf.set('APPLICATION_ID', application_id)

cargo_options = ['--manifest-path', meson.project_source_root() / 'Cargo.toml']
cargo_env = environment()
cargo_env.set('VERSION', meson.project_version())
cargo_env.set('APPLICATION_ID', application_id)
cargo_env.set('GETTEXT_PACKAGE', gettext_package)
cargo_env.set('DATADIR', datadir)
cargo_env.set('LOCALEDIR', localedir)

if get_option('buildtype') == 'release'
  cargo_options += ['--release']
  rust_target = 'release'
else
  rust_target = 'debug'
endif

cargo_build = custom_target('cargo-build',
  build_by_default: true,
  build_always_stale: true,
  output: meson.project_name(),
  console: true,
  install: true,
  install_dir: get_option('bindir'),
  command: [
    'sh', '-c',
    'cargo build @0@ && cp @1@ @OUTPUT@'.format(
      ' '.join(cargo_options),
      meson.project_source_root() / 'target' / rust_target / meson.project_name()
    )
  ],
  env: cargo_env,
)

subdir('po')
subdir('data')

gnome.post_install(
  gtk_update_icon_cache: true,
  glib_compile_schemas: true,
  update_desktop_database: true,
)

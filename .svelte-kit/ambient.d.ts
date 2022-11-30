
// this file is generated — do not edit it


/// <reference types="@sveltejs/kit" />

/**
 * Environment variables [loaded by Vite](https://vitejs.dev/guide/env-and-mode.html#env-files) from `.env` files and `process.env`. Like [`$env/dynamic/private`](https://kit.svelte.dev/docs/modules#$env-dynamic-private), this module cannot be imported into public-facing code. This module only includes variables that _do not_ begin with [`config.kit.env.publicPrefix`](https://kit.svelte.dev/docs/configuration#env).
 * 
 * _Unlike_ [`$env/dynamic/private`](https://kit.svelte.dev/docs/modules#$env-dynamic-private), the values exported from this module are statically injected into your bundle at build time, enabling optimisations like dead code elimination.
 * 
 * ```ts
 * import { API_KEY } from '$env/static/private';
 * ```
 * 
 * Note that all environment variables referenced in your code should be declared (for example in an `.env` file), even if they don't have a value until the app is deployed:
 * 
 * ```
 * MY_FEATURE_FLAG=""
 * ```
 * 
 * You can override `.env` values from the command line like so:
 * 
 * ```bash
 * MY_FEATURE_FLAG="enabled" npm run dev
 * ```
 */
declare module '$env/static/private' {
	export const MANPATH: string;
	export const NIX_PROFILES: string;
	export const TERM_PROGRAM: string;
	export const npm_package_scripts_tauri: string;
	export const NODE: string;
	export const thm_green: string;
	export const thm_bg: string;
	export const INIT_CWD: string;
	export const npm_package_devDependencies_typescript: string;
	export const TERM: string;
	export const SHELL: string;
	export const npm_package_devDependencies_vite: string;
	export const thm_blue: string;
	export const TMPDIR: string;
	export const HOMEBREW_REPOSITORY: string;
	export const thm_magenta: string;
	export const TERM_PROGRAM_VERSION: string;
	export const WINDOWID: string;
	export const npm_package_scripts_dev: string;
	export const npm_package_dependencies__tauri_apps_api: string;
	export const npm_package_private: string;
	export const npm_package_devDependencies__sveltejs_kit: string;
	export const npm_package_devDependencies_svelte_preprocess: string;
	export const npm_config_registry: string;
	export const ZSH: string;
	export const PNPM_HOME: string;
	export const LC_ALL: string;
	export const thm_red: string;
	export const USER: string;
	export const NVM_DIR: string;
	export const npm_package_scripts_check_watch: string;
	export const COMMAND_MODE: string;
	export const ALACRITTY_SOCKET: string;
	export const PNPM_SCRIPT_SRC_DIR: string;
	export const npm_package_devDependencies__sveltejs_adapter_static: string;
	export const SSH_AUTH_SOCK: string;
	export const ALACRITTY_LOG: string;
	export const __CF_USER_TEXT_ENCODING: string;
	export const npm_package_devDependencies_postcss: string;
	export const npm_package_devDependencies_tslib: string;
	export const npm_execpath: string;
	export const thm_gray: string;
	export const PAGER: string;
	export const GO111MODULE: string;
	export const FZF_DEFAULT_OPTS: string;
	export const npm_package_devDependencies_svelte: string;
	export const TMUX: string;
	export const LSCOLORS: string;
	export const TAURI_PLATFORM_TYPE: string;
	export const PATH: string;
	export const npm_config_engine_strict: string;
	export const __CFBundleIdentifier: string;
	export const PWD: string;
	export const npm_package_devDependencies_tailwindcss: string;
	export const npm_package_scripts_preview: string;
	export const EDITOR: string;
	export const npm_lifecycle_event: string;
	export const npm_package_name: string;
	export const NODE_PATH: string;
	export const npm_package_scripts_build: string;
	export const XPC_FLAGS: string;
	export const TMUX_PANE: string;
	export const thm_pink: string;
	export const NIX_SSL_CERT_FILE: string;
	export const TAURI_PLATFORM: string;
	export const npm_config_node_gyp: string;
	export const XPC_SERVICE_NAME: string;
	export const npm_package_version: string;
	export const npm_package_devDependencies_autoprefixer: string;
	export const npm_package_devDependencies_svelte_check: string;
	export const SHLVL: string;
	export const HOME: string;
	export const npm_package_type: string;
	export const tmux_version: string;
	export const TAURI_PLATFORM_VERSION: string;
	export const TAURI_FAMILY: string;
	export const HOMEBREW_PREFIX: string;
	export const thm_fg: string;
	export const thm_cyan: string;
	export const thm_black4: string;
	export const LOGNAME: string;
	export const LESS: string;
	export const ALACRITTY_WINDOW_ID: string;
	export const npm_package_devDependencies__tauri_apps_cli: string;
	export const npm_lifecycle_script: string;
	export const GOPATH: string;
	export const TAURI_ARCH: string;
	export const npm_config_user_agent: string;
	export const INFOPATH: string;
	export const HOMEBREW_CELLAR: string;
	export const thm_orange: string;
	export const thm_yellow: string;
	export const thm_black: string;
	export const is_vim: string;
	export const TAURI_DEBUG: string;
	export const npm_package_scripts_check: string;
	export const COLORTERM: string;
	export const npm_node_execpath: string;
}

/**
 * Similar to [`$env/static/private`](https://kit.svelte.dev/docs/modules#$env-static-private), except that it only includes environment variables that begin with [`config.kit.env.publicPrefix`](https://kit.svelte.dev/docs/configuration#env) (which defaults to `PUBLIC_`), and can therefore safely be exposed to client-side code.
 * 
 * Values are replaced statically at build time.
 * 
 * ```ts
 * import { PUBLIC_BASE_URL } from '$env/static/public';
 * ```
 */
declare module '$env/static/public' {

}

/**
 * This module provides access to runtime environment variables, as defined by the platform you're running on. For example if you're using [`adapter-node`](https://github.com/sveltejs/kit/tree/master/packages/adapter-node) (or running [`vite preview`](https://kit.svelte.dev/docs/cli)), this is equivalent to `process.env`. This module only includes variables that _do not_ begin with [`config.kit.env.publicPrefix`](https://kit.svelte.dev/docs/configuration#env).
 * 
 * This module cannot be imported into public-facing code.
 * 
 * ```ts
 * import { env } from '$env/dynamic/private';
 * console.log(env.DEPLOYMENT_SPECIFIC_VARIABLE);
 * ```
 * 
 * > In `dev`, `$env/dynamic` always includes environment variables from `.env`. In `prod`, this behavior will depend on your adapter.
 */
declare module '$env/dynamic/private' {
	export const env: {
		MANPATH: string;
		NIX_PROFILES: string;
		TERM_PROGRAM: string;
		npm_package_scripts_tauri: string;
		NODE: string;
		thm_green: string;
		thm_bg: string;
		INIT_CWD: string;
		npm_package_devDependencies_typescript: string;
		TERM: string;
		SHELL: string;
		npm_package_devDependencies_vite: string;
		thm_blue: string;
		TMPDIR: string;
		HOMEBREW_REPOSITORY: string;
		thm_magenta: string;
		TERM_PROGRAM_VERSION: string;
		WINDOWID: string;
		npm_package_scripts_dev: string;
		npm_package_dependencies__tauri_apps_api: string;
		npm_package_private: string;
		npm_package_devDependencies__sveltejs_kit: string;
		npm_package_devDependencies_svelte_preprocess: string;
		npm_config_registry: string;
		ZSH: string;
		PNPM_HOME: string;
		LC_ALL: string;
		thm_red: string;
		USER: string;
		NVM_DIR: string;
		npm_package_scripts_check_watch: string;
		COMMAND_MODE: string;
		ALACRITTY_SOCKET: string;
		PNPM_SCRIPT_SRC_DIR: string;
		npm_package_devDependencies__sveltejs_adapter_static: string;
		SSH_AUTH_SOCK: string;
		ALACRITTY_LOG: string;
		__CF_USER_TEXT_ENCODING: string;
		npm_package_devDependencies_postcss: string;
		npm_package_devDependencies_tslib: string;
		npm_execpath: string;
		thm_gray: string;
		PAGER: string;
		GO111MODULE: string;
		FZF_DEFAULT_OPTS: string;
		npm_package_devDependencies_svelte: string;
		TMUX: string;
		LSCOLORS: string;
		TAURI_PLATFORM_TYPE: string;
		PATH: string;
		npm_config_engine_strict: string;
		__CFBundleIdentifier: string;
		PWD: string;
		npm_package_devDependencies_tailwindcss: string;
		npm_package_scripts_preview: string;
		EDITOR: string;
		npm_lifecycle_event: string;
		npm_package_name: string;
		NODE_PATH: string;
		npm_package_scripts_build: string;
		XPC_FLAGS: string;
		TMUX_PANE: string;
		thm_pink: string;
		NIX_SSL_CERT_FILE: string;
		TAURI_PLATFORM: string;
		npm_config_node_gyp: string;
		XPC_SERVICE_NAME: string;
		npm_package_version: string;
		npm_package_devDependencies_autoprefixer: string;
		npm_package_devDependencies_svelte_check: string;
		SHLVL: string;
		HOME: string;
		npm_package_type: string;
		tmux_version: string;
		TAURI_PLATFORM_VERSION: string;
		TAURI_FAMILY: string;
		HOMEBREW_PREFIX: string;
		thm_fg: string;
		thm_cyan: string;
		thm_black4: string;
		LOGNAME: string;
		LESS: string;
		ALACRITTY_WINDOW_ID: string;
		npm_package_devDependencies__tauri_apps_cli: string;
		npm_lifecycle_script: string;
		GOPATH: string;
		TAURI_ARCH: string;
		npm_config_user_agent: string;
		INFOPATH: string;
		HOMEBREW_CELLAR: string;
		thm_orange: string;
		thm_yellow: string;
		thm_black: string;
		is_vim: string;
		TAURI_DEBUG: string;
		npm_package_scripts_check: string;
		COLORTERM: string;
		npm_node_execpath: string;
		[key: string]: string | undefined;
	}
}

/**
 * Similar to [`$env/dynamic/private`](https://kit.svelte.dev/docs/modules#$env-dynamic-private), but only includes variables that begin with [`config.kit.env.publicPrefix`](https://kit.svelte.dev/docs/configuration#env) (which defaults to `PUBLIC_`), and can therefore safely be exposed to client-side code.
 * 
 * Note that public dynamic environment variables must all be sent from the server to the client, causing larger network requests — when possible, use `$env/static/public` instead.
 * 
 * ```ts
 * import { env } from '$env/dynamic/public';
 * console.log(env.PUBLIC_DEPLOYMENT_SPECIFIC_VARIABLE);
 * ```
 */
declare module '$env/dynamic/public' {
	export const env: {
		[key: string]: string | undefined;
	}
}

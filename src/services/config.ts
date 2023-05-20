import { type as osType } from '@tauri-apps/api/os'
import { getConfig } from './commands'

export type Config = {
  user_settings: UserSettings
  search_services: SearchServiceConfig[]
  app_settings: AppSettings
}

export type AppSettings = {
  modifier_key: string
  escape_closes_info: boolean
  escape_closes_service_search: boolean
}

export type UserSettings = {
  entries_to_show: number
  fzf_algorithm: string
  similarity: number
  shortcut: string
}

export type SearchServiceConfig = {
  name: string
  shortcut: string
  algorithm: string
  similarity: number
  file_settings: FileSettings
}

export type FileSettings = {
  source_file: string
  file_type: string
  sheet: string
  rows_to_skip: number
  fields: FieldConfig[]
}

export type FieldConfig = {
  name: string
  search: boolean
  display: boolean
  display_name: string
  shortcut: string
  qr_template: string
}

let config: Config

export async function get_config(reload: boolean): Promise<Config> {
  if (reload || !config) {
    config = await getConfig()
  }

  const currentOs = await osType()
  config.app_settings.modifier_key =
    currentOs && currentOs === 'Darwin' ? 'Cmd' : 'Ctrl'

  return config
}

export async function get_services_map(): Promise<
  Map<string, SearchServiceConfig>
> {
  const config = await get_config(false)

  const services_map = new Map<string, SearchServiceConfig>()
  for (let i = 0; i < config.search_services.length; i++) {
    services_map.set(config.search_services[i].name, config.search_services[i])
  }
  return services_map
}

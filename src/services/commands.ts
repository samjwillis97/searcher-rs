import { invoke } from '@tauri-apps/api/tauri'
import type { Config } from './config'
import type { Field } from './searcher'

export async function getConfig(): Promise<Config> {
  return await invoke('get_config')
}

export async function resizeWindow(height: number) {
  return await invoke('resize_window', { height })
}

export async function resizeInfoWindow(height: number) {
  return await invoke('resize_info_window', { height })
}

export async function closeSearch() {
  return await invoke('close_search')
}

export async function closeWindow() {
  return await invoke('close_window')
}

export async function openService(service: string) {
  return await invoke('open_service', { service })
}

export async function openPreviousService() {
  return await invoke('open_previous_service')
}

export async function openInfo(id: string) {
  return await invoke('open_info', { id })
}

export async function getInfo(id: string): Promise<Field[]> {
  return invoke('get_info', { id })
}

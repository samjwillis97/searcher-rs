export const ssr = false
export const prerender = true

export type InfoData = {
  service: string
  id: string
}

export const load: InfoData = ({ params }) => {
  return { id: params.slug }
}

export type InfoData = {
  service: string
  id: string
}

export const load: InfoData = ({ params }) => {
  return { id: params.slug }
}

export type InfoData = {
  service: string
  id: string
}

export const load: InfoData = ({ params }) => {
  console.log(params)
  return { id: params.slug }
}

export type InfoData = {
  id: string;
};

export const load: InfoData = ({ params }) => {
  return { id: params.slug };
};

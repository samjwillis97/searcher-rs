// @ts-nocheck
export type InfoData = {
  id: string;
};

export const load = ({ params }: Parameters<InfoData>[0]) => {
  return { id: params.slug };
};

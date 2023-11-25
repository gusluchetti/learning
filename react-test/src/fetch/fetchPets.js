import { API_URL } from "../Constants";

const fetchPets = async ({ queryKey }) => {
  const { animal, location, breed } = queryKey[1];
  const filter = `?animal=${animal}&location=${location}&breed=${breed}`;
  const response = await fetch(`${API_URL}/pets${filter}`);

  if (!response.ok) {
    throw new Error(`search pets with args (${filter}) NOT ok`);
  }

  return response.json();
};

export default fetchPets;

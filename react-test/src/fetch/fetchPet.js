import { API_URL } from "./Constants";

const fetchPet = async ({ queryKey }) => {
  const id = queryKey[1];
  const response = await fetch(`${API_URL}/pets?id=${id}`)

  if (!response.ok) {
    throw new Error(`details/${id} fetch NOT ok`);
  }

  return response.json();
}

export default fetchPet;

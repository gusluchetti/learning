import { useState, useEffect } from "react";
import { API_URL, STATUS } from "./Constants";

const localCache = {};

export default function useBreedList(animal) {
  const [breedList, setBreedList] = useState([]);
  const [status, setStatus] = useState(STATUS.unloaded);

  useEffect(() => {
    if (!animal) {
      setBreedList([]);
    } else if (localCache[animal]) {
      setBreedList(localCache[animal]);
    } else if (!localCache[animal]) {
      requestBreedList();
    }

    async function requestBreedList() {
      setBreedList([]);
      setStatus(STATUS.loading);

      const res = await fetch(`${API_URL}/breeds?animal=${animal}`);
      const json = await res.json();

      localCache[animal] = json.breeds || [];

      setBreedList(localCache[animal]);
      setStatus(STATUS.loaded);
    }
  }, [animal]);

  return [breedList, status];
}

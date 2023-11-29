import { useQuery } from "@tanstack/react-query";
import { useState } from "react";
import { ANIMALS } from "./Constants";

import fetchPets from "./fetch/fetchPets";
import useBreedList from "./useBreedList";

import Pets from "./Pets";

const SearchParams = () => {
  const [requestParams, setRequestParams] = useState({
    animal: "",
    location: "",
    breed: "",
  });
  const [animal, setAnimal] = useState("");
  const [breeds, _status] = useBreedList(animal);
  let pets = [];

  const results = useQuery({
    queryKey: ["search_pets", requestParams],
    queryFn: fetchPets,
  });

  if (results.isSuccess) {
    pets = results.data.pets;
  }

  return (
    <div>
      <form
        onSubmit={(e) => {
          const formData = new FormData(e.target);
          setRequestParams({
            animal: formData.get("animal"),
            location: formData.get("location"),
            breed: formData.get("breed"),
          });
          e.preventDefault();
        }}
        className="search-form"
      >
        <label htmlFor="location">
          Location:
          <input id="location" name="location" />
        </label>

        <label htmlFor="animal">
          Animal:
          <select
            id="animal"
            name="animal"
            onChange={(e) => {
              setAnimal(e.target.value);
            }}
          >
            <option />
            {ANIMALS.map((a) => (
              <option key={a}>{a}</option>
            ))}
          </select>
        </label>

        <label htmlFor="breed">
          Breed:
          <select
            id="breed"
            name="breed"
            disabled={breeds.length === 0}
            // onChange={(e) => setBreed(e.target.value)}
          >
            <option />
            {breeds.map((b) => (
              <option key={b}>{b}</option>
            ))}
          </select>
        </label>

        <button type="submit">Submit</button>
      </form>

      <Pets pets={pets} />
    </div>
  );
};

export default SearchParams;

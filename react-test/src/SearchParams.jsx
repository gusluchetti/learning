import { useState, useEffect } from "react";
import { ANIMALS, API_URL } from "./Constants";
import useBreedList from "./useBreedList";
import Results from "./Results";

const SearchParams = () => {
  // useState == react hook to add state variable
  // state variable = retain data between render,
  // update var and trigger re-render
  const [location, setLocation] = useState("");
  const [animal, setAnimal] = useState("");
  const [breed, setBreed] = useState("");
  const [pets, setPets] = useState([]);
  const [breeds, status] = useBreedList(animal);

  // useEffect = connect and synchronize to any external systems
  useEffect(() => {
    requestPets();
  }, []);

  async function requestPets() {
    const filter = `/pets?animal=${animal}&location=${location}&breed=${breed}`;
    const res = await fetch(`${API_URL}${filter}`);
    const json = await res.json();

    setPets(json.pets);
  }

  return (
    <div>
      <form
        onSubmit={(e) => {
          e.preventDefault();
          requestPets();
        }}
        className="search-form"
      >
        <label htmlFor="location">
          Location:
          <input
            id="location"
            onChange={(e) => setLocation(e.target.value)}
            value={location}
          />
        </label>

        <label htmlFor="animal">
          Animal:
          <select
            id="animal"
            onChange={(e) => {
              setAnimal(e.target.value);
              setBreed("");
            }}
            value={animal}
          >
            <option />
            {ANIMALS.map((x) => (
              <option key={x}>{x}</option>
            ))}
          </select>
        </label>

        <label htmlFor="breed">
          Breed:
          <select
            id="breed"
            disabled={breeds.length === 0}
            onChange={(e) => setBreed(e.target.value)}
            value={breed}
          >
            <option />
            {breeds.map((x) => (
              <option key={x}>{x}</option>
            ))}
          </select>
        </label>

        <button type="submit">Submit</button>
      </form>

      <Results pets={pets} />
    </div>
  );
};

export default SearchParams;

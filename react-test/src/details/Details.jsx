import { useParams } from "react-router-dom";
import { useQuery } from "@tanstack/react-query";
import fetchPet from "../fetchPet";

const Details = () => {
  const { id } = useParams();
  // if details/id not cached, run fetch
  // details and id is queryKey!
  const results = useQuery(["details", id], fetchPet);

  if (results.isLoading || results.isFetching) { // first load
    return (
      <div>
        <h2>Loading...</h2>
      </div>
    )
  }

  const pet = results.data.pets[0];

  return (
    <div>
      <h1>{pet.name}</h1>
      <h2>{pet.animal} - {pet.breed}</h2>
    </div>
  )
};

export default Details;

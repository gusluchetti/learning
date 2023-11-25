import { useParams } from "react-router-dom";
import { useQuery } from "@tanstack/react-query";
import fetchPet from "../fetch/fetchPet";

const Details = () => {
  const { id } = useParams();
  const results = useQuery({ queryKey: ["details", id], queryFn: fetchPet });

  if (results.isLoading || results.isFetching) {
    // first load
    return (
      <div>
        <h2>Loading...</h2>
      </div>
    );
  }

  const pet = results.data.pets[0];

  return (
    <div>
      <h1>{pet.name}</h1>
      <h2>
        {pet.animal} - {pet.breed}
      </h2>
      <p>{pet.description}</p>
    </div>
  );
};

export default Details;

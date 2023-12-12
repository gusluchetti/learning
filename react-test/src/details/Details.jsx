import { useQuery } from "@tanstack/react-query";
import { useContext, useState } from "react";
import { useNavigate, useParams } from "react-router-dom";

import AdoptedPetContext from "../AdoptedPetContext";
import Carousel from "../Carousel";
import ErrorBoundary from "../ErrorBoundary";
import Modal from "../Modal";
import fetchPet from "../fetch/fetchPet";

const Details = () => {
  const [showModal, setShowModal] = useState(false);
  const navigate = useNavigate();
  const [_, setAdoptedPet] = useContext(AdoptedPetContext);
  const { id } = useParams();
  const results = useQuery({ queryKey: ["details", id], queryFn: fetchPet });

  if (results.isLoading || results.isFetching) {
    return (
      <div>
        <h2>Loading...</h2>
      </div>
    );
  }

  const pet = results.data.pets[0];

  return (
    <div className="details">
      <Carousel images={pet.images} />
      <h1>{pet.name}</h1>
      <h2>
        {pet.animal} - {pet.breed}
      </h2>
      <button type="button" onClick={() => setShowModal(true)}>
        Adopt Me!
      </button>
      <p>{pet.description}</p>
      {showModal ? (
        <Modal>
          <div>
            <h1>Are you sure you'd like to adopt {pet.name}?</h1>
          </div>
          <button
            type="button"
            onClick={() => {
              setAdoptedPet(pet);
              navigate("/");
            }}
          >
            Yes
          </button>
          <button type="button" onClick={() => setShowModal(false)}>
            No
          </button>
        </Modal>
      ) : null}
    </div>
  );
};

export default function DetailsErrorBoundary(props) {
  return (
    <ErrorBoundary>
      <Details {...props} />
    </ErrorBoundary>
  );
}

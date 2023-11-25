import { API_URL } from "./Constants";
import { Component } from "react";

class Carousel extends Component {
  state = {
    active: 0
  }

  static defaultProps = {
    images: [`${API_URL}/pets/none.jpg`]
  }

  render() {
    const { active } = this.state;
    const { images } = this.props;

    return (
      <div>
        <img src={images[active]} alt="animal hero" />
        <div>
          {images.map((photo, index) => (
            <img key={photo} src={photo}
              className={index === active ? "active" : ""}
              alt="animal thumbnail"
            />
          ))}
        </div>
      </div>
    )
  }
}

export default Carousel;

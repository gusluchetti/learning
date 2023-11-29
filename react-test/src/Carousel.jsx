import { Component } from "react";
import { API_URL } from "./Constants";

class Carousel extends Component {
  state = {
    active: 0,
  };

  static defaultProps = {
    images: [`${API_URL}/pets/none.jpg`],
  };

  handleIndexClick = (e) => {
    this.setState({
      active: +e.target.dataset.index,
    });
  };

  render() {
    const { active } = this.state;
    const { images } = this.props;

    return (
      <div>
        <img src={images[active]} alt="animal hero" />
        <div>
          {images.map((photo, index) => (
            <img
              src={photo}
              alt="animal thumbnail"
              className={index === active ? "active" : ""}
              key={photo}
              data-index={index}
              onClick={this.handleIndexClick}
            />
          ))}
        </div>
      </div>
    );
  }
}

export default Carousel;

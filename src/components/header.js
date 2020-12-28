import React from 'react';
import '../styles/header.css'

class Header extends React.Component {
  render() {
    return (
      <div class="container">
        <nav class="navbar navbar-expand-lg navbar-light fixed-top">
          <div class="navbar-nav mx-auto">
            <a class="nav-item nav-link" href="#home">Home</a>
            <a class="nav-item nav-link" href="#about">About</a>
            <a class="nav-item nav-link" href="#education">Education</a>
            <a class="nav-item nav-link" href="#experiences">Experiences</a>
            <a class="nav-item nav-link" href="#projects">Projects</a>
          </div>
        </nav>
      </div>
    )
  }
}

export default Header;
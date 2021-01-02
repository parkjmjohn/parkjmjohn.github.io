import React from 'react';
import '../styles/header.css';

const Header = () => {
    return (
        <div className="container">
            <nav className="navbar navbar-expand navbar-light static-top">
                <div className="navbar-nav mx-auto">
                    <a className="nav-item nav-link" href="#about">
                        About
                    </a>
                    <a className="nav-item nav-link" href="#experiences">
                        Experiences
                    </a>
                    <a className="nav-item nav-link" href="#projects">
                        Projects
                    </a>
                    <a className="nav-item nav-link" href="#hobbies">
                        Hobbies
                    </a>
                </div>
            </nav>
        </div>
    );
};

export default Header;

import React from 'react';

import email from '/content/icons/email.svg';
import phone from '/content/icons/phone.svg';
import location from '/content/icons/location.svg';
import prof from '/content/images/prof.heic';
import resume from '/content/resume.pdf';

import '../styles/about.css';

class About extends React.Component {
    render () {
        return (
            <div className="container" id="about">
                <div className="row">
                    <div className="col">
                        <div className="container card-inf">
                            <h4 className="display-4">John J. Park</h4>
                            <h5 className="display-5">Software Engineer</h5>
                            <br />
                            <ul className="list-unstyled">
                                <li>
                                    <img src={email} /> jjp282@cornell.edu
                                </li>
                                <li>
                                    <img src={phone} /> (646) 461 0807
                                </li>
                                <li>
                                    <img src={location} /> Ithaca, NY
                                </li>
                            </ul>
                            <br />
                            <ul className="list-inline">
                                <li className="list-inline-item">
                                    <a
                                        className="btn btn-social-icon btn-github btn-soc"
                                        href="http://github.com/parkjmjohn"
                                    >
                                        <span className="fa fa-github" />
                                    </a>
                                </li>
                                <li className="list-inline-item">
                                    <a
                                        className="btn btn-social-icon btn-linkedin btn-soc "
                                        href="https://www.linkedin.com/in/john-j-park-836914145/"
                                    >
                                        <span className="fa fa-linkedin" />
                                    </a>
                                </li>
                                <li className="list-inline-item">
                                    <a
                                        className="btn btn-social-icon btn-twitter btn-soc"
                                        href="https://twitter.com/jpsportsgeek"
                                    >
                                        <span className="fa fa-twitter" />
                                    </a>
                                </li>
                            </ul>
                        </div>
                    </div>
                    <div className="col">
                        <img src={prof} className="prof-img" />
                    </div>
                </div>
                <div className="row">
                    <div className="col" />
                    <div className="col">
                        <h5 className="display-5">About Me</h5>
                        <p>
							I am an undergraduate at Cornell University expecting to graduate
							in May 2021 with a B.S. in Computer Science. Interests in full time positions.
                        </p>
                        <a
                            href={resume}
                            role="button"
                            className="btn btn-outline-secondary btn-resume"
                            target="_blank"
                            rel="noopener noreferrer"
                        >
							Download CV
                        </a>
                    </div>
                </div>
            </div>
        );
    }
}

export default About;

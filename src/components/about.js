import React from 'react';
import resume from '/content/resume.pdf';
import '../styles/about.css';

class About extends React.Component {
    render () {
        return (
            <div className="container" id="about">
                <div className="row">
                    <div className="col" />
                    <div className="col">
                        <h5 className="display-5">About Me</h5>
                        <p>
							I am an undergraduate at Cornell University expecting to graduate
							in May 2021 with a B.S. in Computer Science. Checkout my resume!
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

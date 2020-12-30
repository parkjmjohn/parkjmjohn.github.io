import React from 'react';
import '../styles/experience.css';

class Experience extends React.Component {
    render () {
        return (
            <div className="container" id="experiences">
                <div className="row">
                    <div className="col">
                        <h5 className="display-5">Experiences</h5>
                    </div>
                </div>
                <div className="row">
                    <div className="col">
                        <ul className="list-unstyled">
                            <li>Asymm Developers</li>
                            <li>San Diego, CA</li>
                        </ul>
                    </div>
                    <div className="col">
                        <ul className="ascii">
                            <li>Technical Product Manager Intern
                                <ul>
                                    <li>less/</li>
                                    <li>js/</li>
                                    <li>fonts/</li>
                                </ul>
                            </li>
                        </ul>
                    </div>
                </div>
            </div>
        );
    }
}

export default Experience;

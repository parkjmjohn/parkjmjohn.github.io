import React from 'react';
import email from '../icons/email.svg'
import phone from '../icons/phone.svg'
import location from '../icons/location.svg'
import prof from '../images/prof.heic'
import '../styles/home.css';

class Home extends React.Component {
  render() {
    return (
      <div className="container">
        <div className="row">
          <div className="col">
            <div className="container card_inf">
              <h4 className="display-4">John J. Park</h4>
              <h5 className="display-5">Software Engineer</h5>
              <br />
              <ul className="list-unstyled">
                <li><img src={email}></img>    jjp282@cornell.edu</li>
                <li><img src={phone}></img>     (646) 461 0807</li>
                <li><img src={location}></img>    Ithaca, NY</li>
              </ul>
              <br />
              <ul className="list-inline">
                <li className="list-inline-item">
                  <a className="btn btn-social-icon btn-github" href="http://github.com/parkjmjohn">
                    <span className="fa fa-github"></span>
                  </a>
                </li>
                <li className="list-inline-item">
                  <a className="btn btn-social-icon btn-linkedin" href="https://www.linkedin.com/in/john-j-park-836914145/">
                    <span className="fa fa-linkedin"></span>
                  </a>
                </li>
                <li className="list-inline-item">
                  <a className="btn btn-social-icon btn-twitter" href="">
                    <span className="fa fa-twitter"></span>
                  </a>
                </li>
              </ul>
            </div>
          </div>
          <div className="col">
            <img src={prof} className="prof_img"></img>
          </div>
        </div >
      </div >
    )
  }
}

export default Home;
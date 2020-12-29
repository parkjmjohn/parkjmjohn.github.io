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
        <div class="row">
          <div class="col">
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
              <ul class="list-inline">
                <li class="list-inline-item">
                  <a class="btn btn-social-icon btn-github" href="http://github.com/parkjmjohn">
                    <span class="fa fa-github"></span>
                  </a>
                </li>
                <li class="list-inline-item">
                  <a class="btn btn-social-icon btn-linkedin" href="https://www.linkedin.com/in/john-j-park-836914145/">
                    <span class="fa fa-linkedin"></span>
                  </a>
                </li>
                <li class="list-inline-item">
                  <a class="btn btn-social-icon btn-twitter" href="">
                    <span class="fa fa-twitter"></span>
                  </a>
                </li>
              </ul>
            </div>
          </div>
          <div class="col">
            <img src={prof} class="prof_img"></img>
          </div>
        </div >
      </div >
    )
  }
}

export default Home;
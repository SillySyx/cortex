import React from 'react';

export class MainPage extends React.Component {
   constructor(props) {
      super(props);

      this.state = {
         modules: loadModules(),
         selectedModule: "",
         passwords: [],
      };
   }

   componentDidMount() {
      if (!this.state.selectedModule) {
         this.selectModule("passwords");
      }
   }
   
   selectModule(id) {
      console.log("selectModule", id);

      if (this.state.selectedModule === id)
         return;

      if (id === "passwords") {
         this.setState({selectedModule: id, passwords: []});
      }
   }

   render() {
      return (
         <div className="main-layout main-background animation-fade">
            <aside className="main-menu">
               <img className="main-menu-logo" src="icons/brain.svg" alt="" />

               {this.state.modules.map((module, index) => (
                  <img key={index}
                     className={this.state.selectedModule === module.id ? "main-button active" : "main-button"} 
                     src={module.icon} 
                     alt={module.name} 
                     onClick={() => this.selectModule(module.id)} />
               ))}
            </aside>

            <header className="main-header">
               <input className="main-search-box" placeholder="Search for passwords" />
               <img className="main-button main-header-button" src="icons/add.svg" alt="" />
            </header>

            <section className="main-content">
               <h1 className="category-title">Work</h1>
               <div className="category">
                  <div className="password">
                     <h1 className="password-title">Password name</h1>
                     <p className="password-description">email@domain.com</p>
                     <img className="main-button password-icon" src="icons/add.svg" alt="" />
                  </div>
                  <div className="password">
                     <h1 className="password-title">Password name</h1>
                     <p className="password-description">email@domain.com</p>
                     <img className="main-button password-icon" src="icons/add.svg" alt="" />
                  </div>
                  <div className="password">
                     <h1 className="password-title">Password name</h1>
                     <p className="password-description">email@domain.com</p>
                     <img className="main-button password-icon" src="icons/add.svg" alt="" />
                  </div>
               </div>

               <h1 className="category-title">Games</h1>
               <div className="category">
                  <div className="password">
                     <h1 className="password-title">Password name</h1>
                     <p className="password-description">email@domain.com</p>
                     <img className="main-button password-icon" src="icons/add.svg" alt="" />
                  </div>
                  <div className="password">
                     <h1 className="password-title">Password name</h1>
                     <p className="password-description">email@domain.com</p>
                     <img className="main-button password-icon" src="icons/add.svg" alt="" />
                  </div>
               </div>

               <h1 className="category-title">Personal</h1>
               <div className="category">
                  <div className="password">
                     <h1 className="password-title">Password name</h1>
                     <p className="password-description">email@domain.com</p>
                     <img className="main-button password-icon" src="icons/add.svg" alt="" />
                  </div>
                  <div className="password">
                     <h1 className="password-title">Password name</h1>
                     <p className="password-description">email@domain.com</p>
                     <img className="main-button password-icon" src="icons/add.svg" alt="" />
                  </div>
                  <div className="password">
                     <h1 className="password-title">Password name</h1>
                     <p className="password-description">email@domain.com</p>
                     <img className="main-button password-icon" src="icons/add.svg" alt="" />
                  </div>
                  <div className="password">
                     <h1 className="password-title">Password name</h1>
                     <p className="password-description">email@domain.com</p>
                     <img className="main-button password-icon" src="icons/add.svg" alt="" />
                  </div>
                  <div className="password">
                     <h1 className="password-title">Password name</h1>
                     <p className="password-description">email@domain.com</p>
                     <img className="main-button password-icon" src="icons/add.svg" alt="" />
                  </div>
                  <div className="password">
                     <h1 className="password-title">Password name</h1>
                     <p className="password-description">email@domain.com</p>
                     <img className="main-button password-icon" src="icons/add.svg" alt="" />
                  </div>
               </div>

            </section>
         </div>
      )
   }
}

function loadModules() {
   return [
      {
         id: "passwords",
         name: "Passwords",
         icon: "icons/password.svg",
      },
      {
         id: "knowledge",
         name: "Knowledge",
         icon: "icons/knowledge.svg",
      },
   ]
}

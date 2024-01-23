function openPopup() {
    var form = document.getElementById('sectionId');
    var about = document.getElementsByClassName("about")
    form.style.display = 'block';
    about.style.display = "none";
    scrollToSection('sectionId');
  }

/*   function scrollToSection(sectionId) {
    var section = document.getElementById(sectionId);
    if (section && typeof section.scrollIntoView === 'function') {
      section.scrollIntoView({ behavior: 'smooth' });
    }
  }       ISLEMIIIIIIR*/


  function validateForm() {
    var name = document.getElementById('nameInput').value.trim();
    var surname = document.getElementById('surnameInput').value.trim();
    var email = document.getElementById('emailInput').value.trim();
    var phone = document.getElementById('phoneInput').value.trim();

    clearErrors();

    var isValid = true;

    if (name === '') {
      displayError('nameError', 'Please fill in the Name field.');
      isValid = false;
    }

    if (surname === '') {
      displayError('surnameError', 'Please fill in the Surname field.');
      isValid = false;
    }

    if (email === '' || !isValidEmail(email)) {
      displayError('emailError', 'Please fill in a valid Email address.');
      isValid = false;
    }

    if (phone === '' ) {
      displayError('phoneError', 'Please fill in the Phone field.');
      isValid = false;
    }

    return isValid;
  }

  function isValidEmail(email) {
    var emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
    return emailRegex.test(email);
  }

  function isValidPhone(phone) {
    var phoneRegex = /^[0-9]{10}$/;
    return phoneRegex.test(phone);
  }

  function displayError(errorId, errorMessage) {
    var errorElement = document.getElementById(errorId);
    if (errorElement) {
      errorElement.textContent = errorMessage;
    }
  }

  function clearErrors() {
    var errorElements = document.querySelectorAll('.error-message');
    errorElements.forEach(function (element) {
      element.textContent = '';
    });
  }
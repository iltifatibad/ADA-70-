// function openNav() {
//     document.getElementById("coursessettings").style.width = "550px";
//     document.getElementById("coursessettings").style.position = "absolute"
//     document.getElementById("coursessettings").style.marginLeft = "20%";
//     document.getElementById("coursessettings").style.marginTop = "10%";
//     document.getElementById("coursessettings").style.backgroundColor = "red";
//   }
  
//   /* Set the width of the side navigation to 0 */
//   function closeNav() {
//     document.getElementById("mySidenav").style.width = "0";
//   } 

// document.addEventListener("DOMContentLoaded", function () {
//     // Örnek haberleri ekle
//     const newsSection = document.getElementById("news");

//     const newsData = [
//         { title: "Başlık 1", content: "İçerik 1" },
//         { title: "Başlık 2", content: "İçerik 2" },
//         { title: "Başlık 3", content: "İçerik 3" },
//     ];

//     newsData.forEach((item) => {
//         const article = document.createElement("article");
//         const title = document.createElement("h2");
//         const content = document.createElement("p");

//         title.textContent = item.title;
//         content.textContent = item.content;

//         article.appendChild(title);
//         article.appendChild(content);

//         newsSection.appendChild(article);
//     });
// });



document.addEventListener('DOMContentLoaded', function () {
    var swiper = new Swiper('.swiper-container', {
        slidesPerView: 2,
        spaceBetween: 20,
        breakpoints: {
            100: {
                slidesPerView: 1
            },
            576: {
                slidesPerView: 2
            },
            798: {
                slidesPerView: 3
            },
            992: {
                slidesPerView: 4
            }
        },
        navigation: {
            nextEl: '.swiper-button-next',
            prevEl: '.swiper-button-prev'
        },
        autoplay: {
            delay: 5000,
            disableOnInteraction: false
        }
    });
  
});


let slideIndex = 1;
showSlides(slideIndex);function plusSlide(n) {
  showSlides(slideIndex += n);
}function showSlides(n) {
  let i;
  const slides = document.getElementsByClassName("slide");
  if (n > slides.length) {slideIndex = 1}
  if (n < 1) {slideIndex = slides.length}
  for (i = 0; i < slides.length; i++) {
    slides[i].style.display = "none";
  }
  slides[slideIndex-1].style.display = "block";
}function prevSlide() {
  plusSlide(-1);
}function nextSlide() {
  plusSlide(1);
}



// document.addEventListener('DOMContentLoaded', function () {
//   var swiper = new Swiper('.swiper-container', {
//       slidesPerView: 2,
//       spaceBetween: 20,
//       breakpoints: {
//           100: {
//               slidesPerView: 1
//           },
//           576: {
//               slidesPerView: 2
//           },
//           798: {
//               slidesPerView: 3
//           },
//           992: {
//               slidesPerView: 4
//           }
//       },
//       navigation: {
//           nextEl: '.swiper-button-next',
//           prevEl: '.swiper-button-prev'
//       },
//       autoplay: {
//           delay: 5000,
//           disableOnInteraction: false
//       }
//   });

// });

/* Created by Tivotal */

let sideMenu = document.querySelectorAll(".nav-link");
sideMenu.forEach((item) => {
  let li = item.parentElement;

  item.addEventListener("click", () => {
    sideMenu.forEach((link) => {
      link.parentElement.classList.remove("active");
    });
    li.classList.add("active");
  });
});

let menuBar = document.querySelector(".menu-btn");
let sideBar = document.querySelector(".sidebar");
menuBar.addEventListener("click", () => {
  sideBar.classList.toggle("hide");
});

let switchMode = document.getElementById("switch-mode");
switchMode.addEventListener("change", (e) => {
  if (e.target.checked) {
    document.body.classList.add("dark");
  } else {
    document.body.classList.remove("dark");
  }
});

let searchFrom = document.querySelector(".content nav form");
let searchBtn = document.querySelector(".search-btn");
let searchIcon = document.querySelector(".search-icon");
searchBtn.addEventListener("click", (e) => {
  if (window.innerWidth < 576) {
    e.preventDefault();
    searchFrom.classList.toggle("show");
    if (searchFrom.classList.contains("show")) {
      searchIcon.classList.replace("fa-search", "fa-times");
    } else {
      searchIcon.classList.replace("fa-times", "fa-search");
    }
  }
});

window.addEventListener("resize", () => {
  if (window.innerWidth > 576) {
    searchIcon.classList.replace("fa-times", "fa-search");
    searchFrom.classList.remove("show");
  }
  if (window.innerWidth < 768) {
    sideBar.classList.add("hide");
  }
});

if (window.innerWidth < 768) {
  sideBar.classList.add("hide");
}


// mobile 沉浸式导航， 使用jquery实现
// https://www.cnblogs.com/dravenT1/p/9155849.html
/*评论模块的滚动隐藏效果*/
var windowTop=0;//初始话可视区域距离页面顶端的距离
$(window).scroll(function() {
    var scrolls = $(this).scrollTop();//获取当前可视区域距离页面顶端的距离
    if(scrolls>=windowTop){//当scrolls>windowTop时，表示页面在向下滑动
        //需要执行隐藏导航的操作
        if (!$('.more-comment-list-block').hasClass('fadeOutUp')) {
            $('.more-comment-list-block').addClass('animated fadeOutUp');
            $('.more-comment-list-block').removeClass('fadeInDown');
            $('.container').css('margin-top','45px');
        }
        windowTop=scrolls;
    }else{
        //需要执行显示导航动画操作
        if (!$('.more-comment-list-block').hasClass('fadeInDown')) {
            $('.more-comment-list-block').addClass('animated fadeInDown');
            $('.more-comment-list-block').removeClass('fadeOutUp');
            $('.container').css('margin-top','148px');
        }
        windowTop=scrolls;
    }
});
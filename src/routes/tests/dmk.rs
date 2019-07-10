use rocket::Route;
use rocket_contrib::templates::Template;
use lazy_static::lazy_static;
use scraper::{Selector, Html};

pub fn routes() -> Vec<Route> {
  routes![
    fetch_google,
    fetch_dmk_home,
    parse_html,
  ]
}

#[derive(Serialize)]
struct FetchGoogleTemplateData<'a> {
  text: &'a str
}

#[get("/tests/dmk/fetch_google")]
fn fetch_google() -> Template {

  // First fetch google and get html text string
  let url = "https://google.com";
  let mut response = reqwest::get(url).unwrap();
  let html_text = response.text().unwrap();

  // Then parse it to dom tree
  let document = Html::parse_document(&html_text);

  // Use selector to get some element
  lazy_static! {
    static ref BODY_SEL : Selector = Selector::parse("body").unwrap();
  }
  let body = document.select(&BODY_SEL).next().unwrap();

  lazy_static! {
    static ref LOGO_SEL : Selector = Selector::parse("#hplogo").unwrap();
  }
  let logo = body.select(&LOGO_SEL).next().unwrap();
  let logo_src = logo.value().attr("src").unwrap();

  // Render the template
  Template::render("tests/dmk/fetch_google", FetchGoogleTemplateData {
    text: logo_src
  })
}

#[derive(Serialize)]
struct FetchDmkHomeTemplateData<'a> {
  text: &'a String
}

#[get("/tests/dmk/fetch_dmk_home")]
fn fetch_dmk_home() -> Template {
  let url = "https://cartoonmad.com";
  let mut response = reqwest::get(url).unwrap();
  let html_text = response.text_with_charset("big5").unwrap();

  let complete_html_text = format!("<!DOCTYPE html>{}", html_text);

  // Then parse it to dom tree
  let document = Html::parse_document(&complete_html_text);

  // Render the template
  Template::render("tests/dmk/fetch_dmk_home", FetchDmkHomeTemplateData {
    text: &complete_html_text
  })
}

#[get("/tests/dmk/parse_html")]
fn parse_html() -> Template {
  let html_text = "
<html>
<head>
<title>
動漫狂 - 免費動畫漫畫分享社群 !
</title>
<meta name=\"Keywords\" content=\"動漫狂 - 免費動畫漫畫分享社群 ! cartoonmad - 這裡是蘊藏豐富卡通動漫畫的動漫狂,免費卡通,動漫畫,分享下載與在線上觀看 !\"/>
<meta name=\"description\" content=\"動漫狂 - 免費動畫漫畫分享社群 ! cartoonmad - 免費卡通動漫畫,分享下載與在線上觀看 !\"/>
<meta http-equiv=\"Content-Type\" content=\"text/html; charset=big5\">
<meta name=\"robots\" content=\"index,follow\">
<meta name=\"revisit-after\" content=\"1 days\">
<meta name=\"distribution\" content=\"GLOBAL\">
<meta name=\"rating\" content=\"general\">
<link rel=\"alternate\" type=\"application/rss+xml\" title=\"動漫狂 RSS\" href=\"http://www.cartoonmad.com/feed/rss.xml\">
<script>
<!--
if (self != top) top.location.href = window.location.href

function MM_preloadImages() { //v3.0
  var d=document; if(d.images){ if(!d.MM_p) d.MM_p=new Array();
    var i,j=d.MM_p.length,a=MM_preloadImages.arguments; for(i=0; i<a.length; i++)
    if (a[i].indexOf(\"#\")!=0){ d.MM_p[j]=new Image; d.MM_p[j++].src=a[i];}}
}

function MM_swapImgRestore() { //v3.0
  var i,x,a=document.MM_sr; for(i=0;a&&i<a.length&&(x=a[i])&&x.oSrc;i++) x.src=x.oSrc;
}

function MM_findObj(n, d) { //v4.0
  var p,i,x;  if(!d) d=document; if((p=n.indexOf(\"?\"))>0&&parent.frames.length) {
    d=parent.frames[n.substring(p+1)].document; n=n.substring(0,p);}
  if(!(x=d[n])&&d.all) x=d.all[n]; for (i=0;!x&&i<d.forms.length;i++) x=d.forms[i][n];
  for(i=0;!x&&d.layers&&i<d.layers.length;i++) x=MM_findObj(n,d.layers[i].document);
  if(!x && document.getElementById) x=document.getElementById(n); return x;
}

function MM_swapImage() { //v3.0
  var i,j=0,x,a=MM_swapImage.arguments; document.MM_sr=new Array; for(i=0;i<(a.length-2);i+=3)
   if ((x=MM_findObj(a[i]))!=null){document.MM_sr[j++]=x; if(!x.oSrc) x.oSrc=x.src; x.src=a[i+2];}
}
//-->
</script>
<style type=\"text/css\">
<!--
body {font-family:'新細明體';font-size:10pt; color: #000000}
table {font-family:'新細明體';font-size:10pt; color: #383838}
a:link {text-decoration: none;color:#000066}
a:visited{text-decoration: none; color:#000066}
a:active{ text-decoration: none; color:#000066}
a:hover {COLOR: #ff0000; TEXT-DECORATION: none}
.a1:hover {COLOR: #ff0000; POSITION: relative; TOP: 1px; TEXT-DECORATION: none}
.a2:link {text-decoration: none;color:#FFFF99}
.a2:visited{text-decoration: none; color:#FFFF99}
.a2:active{ text-decoration: none; color:#FFFF99}
.a2:hover {COLOR: #ff0000; POSITION: relative; TOP: 1px; TEXT-DECORATION: none}
.tmemu:link {text-decoration: none;color:#3B3A39}
.tmemu:visited{text-decoration: none; color:#3B3A39}
.tmemu:active{ text-decoration: none; color:#3B3A39}
.tmemu:hover {COLOR: #ff0000; POSITION: relative; TOP: 1px; TEXT-DECORATION: none}
.pages {BORDER-BOTTOM: #A2A2A2 1px solid; TEXT-ALIGN: center; BORDER-LEFT: #A2A2A2 1px solid; PADDING-BOTTOM: 1px; LINE-HEIGHT: 15px; PADDING-LEFT: 4px; PADDING-RIGHT: 3px; FONT-FAMILY: Arial; BACKGROUND-COLOR: #F3F3F3; HEIGHT: 0pt; MARGIN-LEFT: 4px; FONT-SIZE: 8pt; BORDER-TOP: #A2A2A2 1px solid; BORDER-RIGHT: #A2A2A2 1px solid; TEXT-DECORATION: none; PADDING-TOP: 2px}
.pages:hover {BORDER-BOTTOM: #7B7B7B 1px solid; TEXT-ALIGN: center; BORDER-LEFT: #7B7B7B 1px solid; PADDING-BOTTOM: 1px; LINE-HEIGHT: 15px; PADDING-LEFT: 4px; PADDING-RIGHT: 3px; FONT-FAMILY: Arial; BACKGROUND-COLOR: #F3F3F3; HEIGHT: 0pt; COLOR: #E80000; MARGIN-LEFT: 4px; FONT-SIZE: 8pt; BORDER-TOP: #7B7B7B 1px solid; BORDER-RIGHT: #7B7B7B 1px solid; TEXT-DECORATION: none; PADDING-TOP: 2px}
.onpage {BORDER-BOTTOM: #7B7B7B 1px solid; TEXT-ALIGN: center; BORDER-LEFT: #7B7B7B 1px solid; PADDING-BOTTOM: 1px; LINE-HEIGHT: 15px; BACKGROUND-COLOR: #8C8CC6; PADDING-LEFT: 4px; PADDING-RIGHT: 3px; FONT-FAMILY: Arial; HEIGHT: 0pt; COLOR: #ffffff; MARGIN-LEFT: 3px; FONT-SIZE: 10pt; BORDER-TOP: #7B7B7B 1px solid; FONT-WEIGHT: bold; BORDER-RIGHT: #7B7B7B 1px solid; TEXT-DECORATION: none; PADDING-TOP: 2px}
.gamethumb {border: 3px solid #ffffff;	margin-bottom: 0px;}
.covers{background:url(/image/cover.png) -1px -2px;position: absolute;z-index: 1;margin: 0px 0px;width:123px;height:132px;CURSOR:pointer;}
.covertxt{background:url(/image/cover.png) -1px -134px;position: absolute;z-index: 2;margin: 132px 0px;width:123px;height:31px;text-align:right;}
#dhtmltooltip{position: absolute;left: -300px;width: 133px;border: 1px solid #000000;padding: 3px;background-color: #eeeeee;visibility: hidden;z-index: 100;}
#dhtmlpointer{position:absolute;left: -300px;z-index: 101;visibility: hidden;}
#dhtmlpointer2{position:absolute;left: -300px;z-index: 101;visibility: hidden;}
.tooltip {font-size: 12px;}
//-->
</style>
<script type=\"text/javascript\" src='tooltip.js'></script>
</head>
<body topMargin=0 onLoad=\"MM_preloadImages('/cartoonimgs/coimg/member_reg_1.gif')\">
<table border=\"0\" cellpadding=\"0\" align=\"center\">
  <tr>
    <td width=\"230\" valign=\"top\">

<table border=\"0\" cellspacing=\"0\" cellpadding=\"0\" width=\"210\">
  <tr>
    <td><img src=\"/image/logo.gif\" width=\"210\" height=\"115\" usemap=\"#Map\" border=\"0\"></td>
  </tr>
  <tr>
    <td align=\"center\"><img src=\"/image/member_box_s.gif\" width=\"210\" height=\"37\"></td>
  </tr>
  <tr>
    <td background=\"/image/member_boxc.gif\" align=\"center\">
      <table border=\"0\" cellpadding=\"0\" cellspacing=\"0\" width=\"168\">
        <form method=\"POST\" action=\"/search.html\">
          <tr>
            <td width=\"41%\" height=\"36\" valign=\"bottom\">
              <p align=\"right\">關鍵字：
            </td>
            <td width=\"59%\" valign=\"bottom\">
              <input type=\"text\" name=\"keyword\" size=\"12\" maxlength=\"12\" style=\"font-size:9pt\">
            </td>
          </tr>
          <tr>
            <td width=\"41%\" height=\"36\">
              <p align=\"right\">類　型：
            </td>
            <td width=\"59%\">
              <select name=\"searchtype\">
                <option value=\"all\" selected>全部</option>
                <option value=\"ctname\" >- 作品名稱</option>
                <option value=\"ctag\" >- 動漫標籤</option>
                <option value=\"author\" >- 原創作者</option>
              </select>
            </td>
          </tr>
          <tr align=\"center\">
            <td colspan=\"2\" height=\"42\"> &nbsp;
              <input type=\"submit\" value=\"快速搜尋\">
            </td>
          </tr>
        </form>
      </table>
      <a href=/member/add.html><img src=\"/image/member_reg_0.gif\" width=\"171\" height=\"43\" border=\"0\" name=\"Image0\" onMouseOut=\"MM_swapImgRestore()\" onMouseOver=\"MM_swapImage('Image0','','/image/member_reg_1.gif',1)\"></a>
  </tr>
  <tr>
    <td><img src=\"/image/member_boxd.gif\" width=\"210\" height=\"21\"></td>
  </tr>
  <tr>
    <td height=\"8\"></td>
  </tr>
  <tr>
    <td><img src=\"/image/menu_boxe.gif\" width=\"210\" height=\"15\"></td>
  </tr>
  <tr>
    <td background=\"/image/menu_boxc.gif\" align=\"center\"> 漫 畫 分 類　<a href=http://www.cartoonmad.com/feed/rss.xml target=_blank><img src=/image/feed.png width=12 height=13 align=absmiddle border=0 alt=動漫狂RSS></a>
      <table width=\"184\" bgcolor=\"#C1C1C1\">
        <tr>
          <td align=\"center\" bgcolor=\"#DBDBDB\">
            <table width=\"164\">
              <tr>
                <td height=\"2\" colspan=\"4\"></td>
              </tr>
              <tr>
                <td height=\"20\" align=\"center\"><a href=\"/comic01.html\">格鬥</a></td>
                <td align=\"center\"><a href=\"/comic02.html\">魔法</a></td>
                <td align=\"center\"><a href=\"/comic03.html\">偵探</a></td>
                <td align=\"center\"><a href=\"/comic04.html\">競技</a></td>
              </tr>
              <tr>
                <td height=\"20\" align=\"center\"><a href=\"/comic10.html\">恐怖</a></td>
                <td align=\"center\"><a href=\"/comic07.html\">戰國</a></td>
                <td align=\"center\"><a href=\"/comic08.html\">魔幻</a></td>
                <td align=\"center\"><a href=\"/comic09.html\">冒險</a></td>
              </tr>
              <tr>
                <td height=\"20\" align=\"center\"><a href=\"/comic16.html\">校園</a></td>
                <td align=\"center\"><a href=\"/comic17.html\">搞笑</a></td>
                <td align=\"center\"><a href=\"/comic13.html\">少女</a></td>
                <td align=\"center\"><a href=\"/comic14.html\">少男</a></td>
              </tr>
              <tr>
                <td height=\"20\" align=\"center\"><a href=\"/comic18.html\">科幻</a></td>
                <td align=\"center\"><a href=\"/comic21.html\">港產</a></td>
                <td align=\"center\"><a href=\"/comic22.html\">其他</a></td>
                <td align=\"center\">&nbsp;</td>
              </tr>
            </table>
          </td>
        </tr>
      </table>
      <table width=\"184\">
        <tr>
          <td height=\"1\"></td>
        </tr>
      </table>
    </td>
  </tr>
  <tr>
    <td><img src=\"/image/menu_boxd.gif\" width=\"210\" height=\"15\"></td>
  </tr>

  <tr>
    <td height=\"8\"></td>
  </tr>
  <td background=\"/image/menu_box1.gif\" height=\"30\" valign=\"bottom\" align=\"center\">
    <table width=\"180\" cellpadding=\"0\" cellspacing=\"0\" border=\"0\">
      <tr>
        <td width=\"34%\">
          <div align=\"center\"><font color=\"#000066\">更新日誌</font></div>
        </td>
        <td width=\"33%\" align=\"right\"> <a href=\"/newcm.html\">最新漫畫</a> </td>
        <td width=\"33%\" align=\"right\"> <a href=\"/comic99.html\">全部漫畫</a> </td>
      </tr>
    </table>
  </td>
  <tr>
    <td background=\"/image/member_boxc.gif\">
      <table width=\"186\" align=\"center\" cellpadding=\"0\" cellspacing=\"0\" border=\"0\">

        <tr onmouseover=\"this.style.backgroundColor='#eeeeee'\" onmouseout=\"this.style.backgroundColor=''\">
          <td align=\"center\" width=\"23%\" height=\"28\">7-10</td>
          <td>

            <div style=\"overflow:hidden;\"><a href=/comic/61520101.html onMouseOver=\"ddrivetip('<table class=tooltip><tr><td><img src=/cartoonimgs/coimgc/6152c.jpg width=120 height=160 class=gamethumb></td></tr><tr><td align=center>異世界精靈的奴... 101 話</td></tr></table>');\" onMouseOut=\"hideddrivetip();\" target=_blank>異世界精靈的奴... 101
              話</a></div>

          </td>
        </tr>
        <tr>
          <td colspan=\"2\"><img src=\"/image/member_boxc2.gif\" width=\"182\" height=\"2\"></td>
        </tr>

        <tr onmouseover=\"this.style.backgroundColor='#eeeeee'\" onmouseout=\"this.style.backgroundColor=''\">
          <td align=\"center\" width=\"23%\" height=\"28\">7-10</td>
          <td>

            <div style=\"overflow:hidden;\"><a href=/comic/57330032.html onMouseOver=\"ddrivetip('<table class=tooltip><tr><td><img src=/cartoonimgs/coimgc/5733c.jpg width=120 height=160 class=gamethumb></td></tr><tr><td align=center>戰×戀 32 話</td></tr></table>');\" onMouseOut=\"hideddrivetip();\" target=_blank>戰×戀漫畫 32
              話</a></div>

          </td>
        </tr>
        <tr>
          <td colspan=\"2\"><img src=\"/image/member_boxc2.gif\" width=\"182\" height=\"2\"></td>
        </tr>

        <tr onmouseover=\"this.style.backgroundColor='#eeeeee'\" onmouseout=\"this.style.backgroundColor=''\">
          <td align=\"center\" width=\"23%\" height=\"28\">7-10</td>
          <td>

            <div style=\"overflow:hidden;\"><a href=/comic/46590023.html onMouseOver=\"ddrivetip('<table class=tooltip><tr><td><img src=/cartoonimgs/coimgc/4659c.jpg width=120 height=160 class=gamethumb></td></tr><tr><td align=center>異世創生錄  23 話</td></tr></table>');\" onMouseOut=\"hideddrivetip();\" target=_blank>異世創生錄 漫畫 23
              話</a></div>

          </td>
        </tr>
        <tr>
          <td colspan=\"2\"><img src=\"/image/member_boxc2.gif\" width=\"182\" height=\"2\"></td>
        </tr>

        <tr onmouseover=\"this.style.backgroundColor='#eeeeee'\" onmouseout=\"this.style.backgroundColor=''\">
          <td align=\"center\" width=\"23%\" height=\"28\">7-10</td>
          <td>

            <div style=\"overflow:hidden;\"><a href=/comic/46400108.html onMouseOver=\"ddrivetip('<table class=tooltip><tr><td><img src=/cartoonimgs/coimgc/4640c.jpg width=120 height=160 class=gamethumb></td></tr><tr><td align=center>寄宿學校的朱麗... 108 話</td></tr></table>');\" onMouseOut=\"hideddrivetip();\" target=_blank>寄宿學校的朱麗... 108
              話</a></div>

          </td>
        </tr>
        <tr>
          <td colspan=\"2\"><img src=\"/image/member_boxc2.gif\" width=\"182\" height=\"2\"></td>
        </tr>

        <tr onmouseover=\"this.style.backgroundColor='#eeeeee'\" onmouseout=\"this.style.backgroundColor=''\">
          <td align=\"center\" width=\"23%\" height=\"28\">7-10</td>
          <td>

            <div style=\"overflow:hidden;\"><a href=/comic/42400231.html onMouseOver=\"ddrivetip('<table class=tooltip><tr><td><img src=/cartoonimgs/coimgc/4240c.jpg width=120 height=160 class=gamethumb></td></tr><tr><td align=center>家有女友 231 話</td></tr></table>');\" onMouseOut=\"hideddrivetip();\" target=_blank>家有女友漫畫 231
              話</a></div>

          </td>
        </tr>
        <tr>
          <td colspan=\"2\"><img src=\"/image/member_boxc2.gif\" width=\"182\" height=\"2\"></td>
        </tr>

        <tr onmouseover=\"this.style.backgroundColor='#eeeeee'\" onmouseout=\"this.style.backgroundColor=''\">
          <td align=\"center\" width=\"23%\" height=\"28\">7-10</td>
          <td>

            <div style=\"overflow:hidden;\"><a href=/comic/32960027.html onMouseOver=\"ddrivetip('<table class=tooltip><tr><td><img src=/cartoonimgs/coimgc/3296c.jpg width=120 height=160 class=gamethumb></td></tr><tr><td align=center>卡片少女召喚脫... 27 話</td></tr></table>');\" onMouseOut=\"hideddrivetip();\" target=_blank>卡片少女召喚脫... 27
              話</a></div>

          </td>
        </tr>
        <tr>
          <td colspan=\"2\"><img src=\"/image/member_boxc2.gif\" width=\"182\" height=\"2\"></td>
        </tr>

        <tr onmouseover=\"this.style.backgroundColor='#eeeeee'\" onmouseout=\"this.style.backgroundColor=''\">
          <td align=\"center\" width=\"23%\" height=\"28\">7-10</td>
          <td>

            <div style=\"overflow:hidden;\"><a href=/comic/17000109.html onMouseOver=\"ddrivetip('<table class=tooltip><tr><td><img src=/cartoonimgs/coimgc/1700c.jpg width=120 height=160 class=gamethumb></td></tr><tr><td align=center>KARNEVA... 109 話</td></tr></table>');\" onMouseOut=\"hideddrivetip();\" target=_blank>KARNEVA... 109
              話</a></div>

          </td>
        </tr>
        <tr>
          <td colspan=\"2\"><img src=\"/image/member_boxc2.gif\" width=\"182\" height=\"2\"></td>
        </tr>

        <tr onmouseover=\"this.style.backgroundColor='#eeeeee'\" onmouseout=\"this.style.backgroundColor=''\">
          <td align=\"center\" width=\"23%\" height=\"28\">7-10</td>
          <td>

            <div style=\"overflow:hidden;\"><a href=/comic/82030006.html onMouseOver=\"ddrivetip('<table class=tooltip><tr><td><img src=/cartoonimgs/coimgc/8203c.jpg width=120 height=160 class=gamethumb></td></tr><tr><td align=center>狼不會入眠 6 話</td></tr></table>');\" onMouseOut=\"hideddrivetip();\" target=_blank>狼不會入眠漫畫 6
              話</a></div>

          </td>
        </tr>
        <tr>
          <td colspan=\"2\"><img src=\"/image/member_boxc2.gif\" width=\"182\" height=\"2\"></td>
        </tr>

        <tr onmouseover=\"this.style.backgroundColor='#eeeeee'\" onmouseout=\"this.style.backgroundColor=''\">
          <td align=\"center\" width=\"23%\" height=\"28\">7-10</td>
          <td>

            <div style=\"overflow:hidden;\"><a href=/comic/81220007.html onMouseOver=\"ddrivetip('<table class=tooltip><tr><td><img src=/cartoonimgs/coimgc/8122c.jpg width=120 height=160 class=gamethumb></td></tr><tr><td align=center>惡役千金流放後... 7 話</td></tr></table>');\" onMouseOut=\"hideddrivetip();\" target=_blank>惡役千金流放後... 7
              話</a></div>

          </td>
        </tr>
        <tr>
          <td colspan=\"2\"><img src=\"/image/member_boxc2.gif\" width=\"182\" height=\"2\"></td>
        </tr>

        <tr onmouseover=\"this.style.backgroundColor='#eeeeee'\" onmouseout=\"this.style.backgroundColor=''\">
          <td align=\"center\" width=\"23%\" height=\"28\">7-10</td>
          <td>

            <div style=\"overflow:hidden;\"><a href=/comic/79070052.html onMouseOver=\"ddrivetip('<table class=tooltip><tr><td><img src=/cartoonimgs/coimgc/7907c.jpg width=120 height=160 class=gamethumb></td></tr><tr><td align=center>伊甸星原 52 話</td></tr></table>');\" onMouseOut=\"hideddrivetip();\" target=_blank>伊甸星原漫畫 52
              話</a></div>

          </td>
        </tr>
        <tr>
          <td colspan=\"2\"><img src=\"/image/member_boxc2.gif\" width=\"182\" height=\"2\"></td>
        </tr>

        <tr onmouseover=\"this.style.backgroundColor='#eeeeee'\" onmouseout=\"this.style.backgroundColor=''\">
          <td align=\"center\" width=\"23%\" height=\"28\">7-10</td>
          <td>

            <div style=\"overflow:hidden;\"><a href=/comic/78850018.html onMouseOver=\"ddrivetip('<table class=tooltip><tr><td><img src=/cartoonimgs/coimgc/7885c.jpg width=120 height=160 class=gamethumb></td></tr><tr><td align=center>國八分 18 話</td></tr></table>');\" onMouseOut=\"hideddrivetip();\" target=_blank>國八分漫畫 18
              話</a></div>

          </td>
        </tr>
        <tr>
          <td colspan=\"2\"><img src=\"/image/member_boxc2.gif\" width=\"182\" height=\"2\"></td>
        </tr>

        <tr onmouseover=\"this.style.backgroundColor='#eeeeee'\" onmouseout=\"this.style.backgroundColor=''\">
          <td align=\"center\" width=\"23%\" height=\"28\">7-10</td>
          <td>

            <div style=\"overflow:hidden;\"><a href=/comic/77420012.html onMouseOver=\"ddrivetip('<table class=tooltip><tr><td><img src=/cartoonimgs/coimgc/7742c.jpg width=120 height=160 class=gamethumb></td></tr><tr><td align=center>戀愛雙人組 12 話</td></tr></table>');\" onMouseOut=\"hideddrivetip();\" target=_blank>戀愛雙人組漫畫 12
              話</a></div>

          </td>
        </tr>
        <tr>
          <td colspan=\"2\"><img src=\"/image/member_boxc2.gif\" width=\"182\" height=\"2\"></td>
        </tr>

        <tr onmouseover=\"this.style.backgroundColor='#eeeeee'\" onmouseout=\"this.style.backgroundColor=''\">
          <td align=\"center\" width=\"23%\" height=\"28\">7-10</td>
          <td>

            <div style=\"overflow:hidden;\"><a href=/comic/76700016.html onMouseOver=\"ddrivetip('<table class=tooltip><tr><td><img src=/cartoonimgs/coimgc/7670c.jpg width=120 height=160 class=gamethumb></td></tr><tr><td align=center>原最強劍士憧憬... 16 話</td></tr></table>');\" onMouseOut=\"hideddrivetip();\" target=_blank>原最強劍士憧憬... 16
              話</a></div>

          </td>
        </tr>
        <tr>
          <td colspan=\"2\"><img src=\"/image/member_boxc2.gif\" width=\"182\" height=\"2\"></td>
        </tr>

        <tr onmouseover=\"this.style.backgroundColor='#eeeeee'\" onmouseout=\"this.style.backgroundColor=''\">
          <td align=\"center\" width=\"23%\" height=\"28\">7-10</td>
          <td>

            <div style=\"overflow:hidden;\"><a href=/comic/54410135.html onMouseOver=\"ddrivetip('<table class=tooltip><tr><td><img src=/cartoonimgs/coimgc/5441c.jpg width=120 height=160 class=gamethumb></td></tr><tr><td align=center>BEASTAR... 135 話</td></tr></table>');\" onMouseOut=\"hideddrivetip();\" target=_blank>BEASTAR... 135
              話</a></div>

          </td>
        </tr>
        <tr>
          <td colspan=\"2\"><img src=\"/image/member_boxc2.gif\" width=\"182\" height=\"2\"></td>
        </tr>

        <tr onmouseover=\"this.style.backgroundColor='#eeeeee'\" onmouseout=\"this.style.backgroundColor=''\">
          <td align=\"center\" width=\"23%\" height=\"28\">7-10</td>
          <td>

            <div style=\"overflow:hidden;\"><a href=/comic/53480082.html onMouseOver=\"ddrivetip('<table class=tooltip><tr><td><img src=/cartoonimgs/coimgc/5348c.jpg width=120 height=160 class=gamethumb></td></tr><tr><td align=center>兒子可愛過頭的... 82 話</td></tr></table>');\" onMouseOut=\"hideddrivetip();\" target=_blank>兒子可愛過頭的... 82
              話</a></div>

          </td>
        </tr>
        <tr>
          <td colspan=\"2\"><img src=\"/image/member_boxc2.gif\" width=\"182\" height=\"2\"></td>
        </tr>

        <tr onmouseover=\"this.style.backgroundColor='#eeeeee'\" onmouseout=\"this.style.backgroundColor=''\">
          <td align=\"center\" width=\"23%\" height=\"28\">7-10</td>
          <td>

            <div style=\"overflow:hidden;\"><a href=/comic/51290020.html onMouseOver=\"ddrivetip('<table class=tooltip><tr><td><img src=/cartoonimgs/coimgc/5129c.jpg width=120 height=160 class=gamethumb></td></tr><tr><td align=center>青葉同學,請告... 20 話</td></tr></table>');\" onMouseOut=\"hideddrivetip();\" target=_blank>青葉同學,請告... 20
              話</a></div>

          </td>
        </tr>
        <tr>
          <td colspan=\"2\"><img src=\"/image/member_boxc2.gif\" width=\"182\" height=\"2\"></td>
        </tr>

        <tr onmouseover=\"this.style.backgroundColor='#eeeeee'\" onmouseout=\"this.style.backgroundColor=''\">
          <td align=\"center\" width=\"23%\" height=\"28\">7-10</td>
          <td>

            <div style=\"overflow:hidden;\"><a href=/comic/50670027.html onMouseOver=\"ddrivetip('<table class=tooltip><tr><td><img src=/cartoonimgs/coimgc/5067c.jpg width=120 height=160 class=gamethumb></td></tr><tr><td align=center>藤原同學說的大... 27 話</td></tr></table>');\" onMouseOut=\"hideddrivetip();\" target=_blank>藤原同學說的大... 27
              話</a></div>

          </td>
        </tr>
        <tr>
          <td colspan=\"2\"><img src=\"/image/member_boxc2.gif\" width=\"182\" height=\"2\"></td>
        </tr>

        <tr onmouseover=\"this.style.backgroundColor='#eeeeee'\" onmouseout=\"this.style.backgroundColor=''\">
          <td align=\"center\" width=\"23%\" height=\"28\">7-10</td>
          <td>

            <div style=\"overflow:hidden;\"><a href=/comic/46130180.html onMouseOver=\"ddrivetip('<table class=tooltip><tr><td><img src=/cartoonimgs/coimgc/4613c.jpg width=120 height=160 class=gamethumb></td></tr><tr><td align=center>炎炎之消防隊 ... 180 話</td></tr></table>');\" onMouseOut=\"hideddrivetip();\" target=_blank>炎炎之消防隊 ... 180
              話</a></div>

          </td>
        </tr>
        <tr>
          <td colspan=\"2\"><img src=\"/image/member_boxc2.gif\" width=\"182\" height=\"2\"></td>
        </tr>

        <tr onmouseover=\"this.style.backgroundColor='#eeeeee'\" onmouseout=\"this.style.backgroundColor=''\">
          <td align=\"center\" width=\"23%\" height=\"28\">7-10</td>
          <td>

            <div style=\"overflow:hidden;\"><a href=/comic/25040318.html onMouseOver=\"ddrivetip('<table class=tooltip><tr><td><img src=/cartoonimgs/coimgc/2504c.jpg width=120 height=160 class=gamethumb></td></tr><tr><td align=center>七原罪 318 話</td></tr></table>');\" onMouseOut=\"hideddrivetip();\" target=_blank>七原罪漫畫 318
              話</a></div>

          </td>
        </tr>
        <tr>
          <td colspan=\"2\"><img src=\"/image/member_boxc2.gif\" width=\"182\" height=\"2\"></td>
        </tr>

        <tr onmouseover=\"this.style.backgroundColor='#eeeeee'\" onmouseout=\"this.style.backgroundColor=''\">
          <td align=\"center\" width=\"23%\" height=\"28\">7-10</td>
          <td>

            <div style=\"overflow:hidden;\"><a href=/comic/21320056.html onMouseOver=\"ddrivetip('<table class=tooltip><tr><td><img src=/cartoonimgs/coimgc/2132c.jpg width=120 height=160 class=gamethumb></td></tr><tr><td align=center>魔王大人你就收... 56 話</td></tr></table>');\" onMouseOut=\"hideddrivetip();\" target=_blank>魔王大人你就收... 56
              話</a></div>

          </td>
        </tr>
        <tr>
          <td colspan=\"2\"><img src=\"/image/member_boxc2.gif\" width=\"182\" height=\"2\"></td>
        </tr>

        <tr onmouseover=\"this.style.backgroundColor='#eeeeee'\" onmouseout=\"this.style.backgroundColor=''\">
          <td align=\"center\" width=\"23%\" height=\"28\">7-10</td>
          <td>

            <div style=\"overflow:hidden;\"><a href=/comic/14270350.html onMouseOver=\"ddrivetip('<table class=tooltip><tr><td><img src=/cartoonimgs/coimgc/1427c.jpg width=120 height=160 class=gamethumb></td></tr><tr><td align=center>源君物語 350 話</td></tr></table>');\" onMouseOut=\"hideddrivetip();\" target=_blank>源君物語漫畫 350
              話</a></div>

          </td>
        </tr>
        <tr>
          <td colspan=\"2\"><img src=\"/image/member_boxc2.gif\" width=\"182\" height=\"2\"></td>
        </tr>

        <tr onmouseover=\"this.style.backgroundColor='#eeeeee'\" onmouseout=\"this.style.backgroundColor=''\">
          <td align=\"center\" width=\"23%\" height=\"28\">7-10</td>
          <td>

            <div style=\"overflow:hidden;\"><a href=/comic/13870323.html onMouseOver=\"ddrivetip('<table class=tooltip><tr><td><img src=/cartoonimgs/coimgc/1387c.jpg width=120 height=160 class=gamethumb></td></tr><tr><td align=center>賭博墮天錄 和... 323 話</td></tr></table>');\" onMouseOut=\"hideddrivetip();\" target=_blank>賭博墮天錄 和... 323
              話</a></div>

          </td>
        </tr>
        <tr>
          <td colspan=\"2\"><img src=\"/image/member_boxc2.gif\" width=\"182\" height=\"2\"></td>
        </tr>

        <tr onmouseover=\"this.style.backgroundColor='#eeeeee'\" onmouseout=\"this.style.backgroundColor=''\">
          <td align=\"center\" width=\"23%\" height=\"28\">7-10</td>
          <td>

            <div style=\"overflow:hidden;\"><a href=/comic/83010002.html onMouseOver=\"ddrivetip('<table class=tooltip><tr><td><img src=/cartoonimgs/coimgc/8301c.jpg width=120 height=160 class=gamethumb></td></tr><tr><td align=center>狩龍人拉格納 2 話</td></tr></table>');\" onMouseOut=\"hideddrivetip();\" target=_blank>狩龍人拉格納漫畫 2
              話</a></div>

          </td>
        </tr>
        <tr>
          <td colspan=\"2\"><img src=\"/image/member_boxc2.gif\" width=\"182\" height=\"2\"></td>
        </tr>

        <tr onmouseover=\"this.style.backgroundColor='#eeeeee'\" onmouseout=\"this.style.backgroundColor=''\">
          <td align=\"center\" width=\"23%\" height=\"28\">7-10</td>
          <td>

            <div style=\"overflow:hidden;\"><a href=/comic/82950005.html onMouseOver=\"ddrivetip('<table class=tooltip><tr><td><img src=/cartoonimgs/coimgc/8295c.jpg width=120 height=160 class=gamethumb></td></tr><tr><td align=center>黑暗集會 5 話</td></tr></table>');\" onMouseOut=\"hideddrivetip();\" target=_blank>黑暗集會漫畫 5
              話</a></div>

          </td>
        </tr>
        <tr>
          <td colspan=\"2\"><img src=\"/image/member_boxc2.gif\" width=\"182\" height=\"2\"></td>
        </tr>

        <tr onmouseover=\"this.style.backgroundColor='#eeeeee'\" onmouseout=\"this.style.backgroundColor=''\">
          <td align=\"center\" width=\"23%\" height=\"28\">7-10</td>
          <td>

            <div style=\"overflow:hidden;\"><a href=/comic/82700010.html onMouseOver=\"ddrivetip('<table class=tooltip><tr><td><img src=/cartoonimgs/coimgc/8270c.jpg width=120 height=160 class=gamethumb></td></tr><tr><td align=center>被勇者隊伍開除... 10 話</td></tr></table>');\" onMouseOut=\"hideddrivetip();\" target=_blank>被勇者隊伍開除... 10
              話</a></div>

          </td>
        </tr>
        <tr>
          <td colspan=\"2\"><img src=\"/image/member_boxc2.gif\" width=\"182\" height=\"2\"></td>
        </tr>

        <tr onmouseover=\"this.style.backgroundColor='#eeeeee'\" onmouseout=\"this.style.backgroundColor=''\">
          <td align=\"center\" width=\"23%\" height=\"28\">7-10</td>
          <td>

            <div style=\"overflow:hidden;\"><a href=/comic/82550011.html onMouseOver=\"ddrivetip('<table class=tooltip><tr><td><img src=/cartoonimgs/coimgc/8255c.jpg width=120 height=160 class=gamethumb></td></tr><tr><td align=center>沒有名字的怪物... 11 話</td></tr></table>');\" onMouseOut=\"hideddrivetip();\" target=_blank>沒有名字的怪物... 11
              話</a></div>

          </td>
        </tr>
        <tr>
          <td colspan=\"2\"><img src=\"/image/member_boxc2.gif\" width=\"182\" height=\"2\"></td>
        </tr>

        <tr onmouseover=\"this.style.backgroundColor='#eeeeee'\" onmouseout=\"this.style.backgroundColor=''\">
          <td align=\"center\" width=\"23%\" height=\"28\">7-10</td>
          <td>

            <div style=\"overflow:hidden;\"><a href=/comic/82000004.html onMouseOver=\"ddrivetip('<table class=tooltip><tr><td><img src=/cartoonimgs/coimgc/8200c.jpg width=120 height=160 class=gamethumb></td></tr><tr><td align=center>放學後的異世界... 4 話</td></tr></table>');\" onMouseOut=\"hideddrivetip();\" target=_blank>放學後的異世界... 4
              話</a></div>

          </td>
        </tr>
        <tr>
          <td colspan=\"2\"><img src=\"/image/member_boxc2.gif\" width=\"182\" height=\"2\"></td>
        </tr>

        <tr onmouseover=\"this.style.backgroundColor='#eeeeee'\" onmouseout=\"this.style.backgroundColor=''\">
          <td align=\"center\" width=\"23%\" height=\"28\">7-10</td>
          <td>

            <div style=\"overflow:hidden;\"><a href=/comic/81860004.html onMouseOver=\"ddrivetip('<table class=tooltip><tr><td><img src=/cartoonimgs/coimgc/8186c.jpg width=120 height=160 class=gamethumb></td></tr><tr><td align=center>雖然到了異世界... 4 話</td></tr></table>');\" onMouseOut=\"hideddrivetip();\" target=_blank>雖然到了異世界... 4
              話</a></div>

          </td>
        </tr>
        <tr>
          <td colspan=\"2\"><img src=\"/image/member_boxc2.gif\" width=\"182\" height=\"2\"></td>
        </tr>

        <tr onmouseover=\"this.style.backgroundColor='#eeeeee'\" onmouseout=\"this.style.backgroundColor=''\">
          <td align=\"center\" width=\"23%\" height=\"28\">7-10</td>
          <td>

            <div style=\"overflow:hidden;\"><a href=/comic/81420036.html onMouseOver=\"ddrivetip('<table class=tooltip><tr><td><img src=/cartoonimgs/coimgc/8142c.jpg width=120 height=160 class=gamethumb></td></tr><tr><td align=center>登山者與被封印... 36 話</td></tr></table>');\" onMouseOut=\"hideddrivetip();\" target=_blank>登山者與被封印... 36
              話</a></div>

          </td>
        </tr>
        <tr>
          <td colspan=\"2\"><img src=\"/image/member_boxc2.gif\" width=\"182\" height=\"2\"></td>
        </tr>

        <tr onmouseover=\"this.style.backgroundColor='#eeeeee'\" onmouseout=\"this.style.backgroundColor=''\">
          <td align=\"center\" width=\"23%\" height=\"28\">7-10</td>
          <td>

            <div style=\"overflow:hidden;\"><a href=/comic/80600034.html onMouseOver=\"ddrivetip('<table class=tooltip><tr><td><img src=/cartoonimgs/coimgc/8060c.jpg width=120 height=160 class=gamethumb></td></tr><tr><td align=center>Queens ... 34 話</td></tr></table>');\" onMouseOut=\"hideddrivetip();\" target=_blank>Queens ... 34
              話</a></div>

          </td>
        </tr>
        <tr>
          <td colspan=\"2\"><img src=\"/image/member_boxc2.gif\" width=\"182\" height=\"2\"></td>
        </tr>

      </table>
    </td>
  </tr>
  <tr>
    <td><img src=\"/image/menu_boxd.gif\" width=\"210\" height=\"15\"></td>
  </tr>
  <tr>
    <td height=\"8\"></td>
  </tr>
  <tr>
    <td>


<table width=\"210\" border=\"0\" cellspacing=\"0\" cellpadding=\"0\" align=\"center\">
  <tr>
    <td height=\"30\" background=\"/image/menu_box4.gif\" align=\"center\" valign=\"bottom\"><a href=\"https://fun8.us/\" target=\"_blank\"><b><font color=\"#EC7125\" face=\"Arial, Helvetica, sans-serif\">Fun8</font></b></a> &nbsp; 放鬆一下吧 !</td>
  </tr>
  <tr>
    <td background=\"/image/member_boxc.gif\" align=center>
      <table border=\"0\" cellspacing=\"0\" cellpadding=\"7\" align=center>
        <tr><td align=center onmouseover=\"this.style.backgroundColor='#eeeeee'\" onmouseout=\"this.style.backgroundColor=''\"><a href=https://fun8.us/post/2377.html target=_blank><img src=https://fun8.us/gamepic/fun8/2377s.jpg width=160 height=90 border=0 title='你發火時有多大威力'><br>你發火時有多大威力</a></td></tr><tr><td align=center onmouseover=\"this.style.backgroundColor='#eeeeee'\" onmouseout=\"this.style.backgroundColor=''\"><a href=https://fun8.us/post/1388.html target=_blank><img src=https://fun8.us/gamepic/fun8/1388s.jpg width=160 height=90 border=0 title='你是三國的哪個角色'><br>你是三國的哪個角色</a></td></tr><tr><td align=center onmouseover=\"this.style.backgroundColor='#eeeeee'\" onmouseout=\"this.style.backgroundColor=''\"><a href=https://fun8.us/post/1420.html target=_blank><img src=https://fun8.us/gamepic/fun8/1420s.jpg width=160 height=90 border=0 title='看看你的淚點高低'><br>看看你的淚點高低</a></td></tr><tr><td align=center onmouseover=\"this.style.backgroundColor='#eeeeee'\" onmouseout=\"this.style.backgroundColor=''\"><a href=https://fun8.us/post/1667.html target=_blank><img src=https://fun8.us/gamepic/fun8/1667s.jpg width=160 height=90 border=0 title='你與哪個星座誰情深緣淺'><br>你與哪個星座誰情深緣淺</a></td></tr><tr><td align=center onmouseover=\"this.style.backgroundColor='#eeeeee'\" onmouseout=\"this.style.backgroundColor=''\"><a href=https://fun8.us/post/1413.html target=_blank><img src=https://fun8.us/gamepic/fun8/1413s.jpg width=160 height=90 border=0 title='你的心理年齡究竟有多大'><br>你的心理年齡究竟有多大</a></td></tr>
      </table>
    </td>
  </tr>
  <tr>
    <td><img src=\"/image/menu_boxd.gif\" width=\"210\" height=\"15\"></td>
  </tr>
</table>

	</td>
  </tr>

</table>
<map name=\"Map\">
  <area shape=\"rect\" coords=\"3,5,206,110\" href=\"/\" alt=\"動漫狂首頁 !\">
</map>

    </td>
    <td valign=\"top\">
      <table border=\"0\" cellspacing=\"0\" cellpadding=\"0\" width=\"890\">
        <tr>
          <td>

<table cellpadding=\"0\" cellspacing=\"0\" border=\"0\" width=\"890\">
  <tr>

    <td colspan=\"4\" align=\"center\" height=\"50\"><font color=\"#bbbbbb\"><font color=#bbbbbb>這裡是蘊藏豐富線上漫畫的動漫狂，來自各地的網友都在動漫狂分享自己<br>的珍藏。當你不知道看什麼漫畫的時候，到動漫狂一定能滿足你的需求！</font></font><br>
    </td>

    <td colspan=\"2\" align=\"center\"> <img src=\"/image/login.gif\" width=\"18\" height=\"13\" align=\"absmiddle\">

      會員 ： <a href=\"/member/add.html\">註冊</a>｜ <a href=\"/member/login.html\">登入</a>｜ <a href=\"/m/\">行動版</a>

    </td>
              </tr>
  <tr align=\"center\" valign=\"top\">
    <td width=\"149\" background=\"/image/memul.png\" height=\"23\"><A HREF=/ class=tmemu style=\"font-size:15px;font-family:'微軟正黑體';\"><b>首 頁</b></A></td>
    <td width=\"148\" background=\"/image/memul.png\"><A class=tmemu style=\"font-size:15px;font-family:'微軟正黑體';\" HREF=/hotrank.html><b>熱門連載</b></A></td>
    <td width=\"148\" background=\"/image/memul.png\"><A class=tmemu style=\"font-size:15px;font-family:'微軟正黑體';\" HREF=/endcm.html><b>經典完結</b></A></td>
    <td width=\"149\" background=\"/image/memul.png\"><A class=tmemu style=\"font-size:15px;font-family:'微軟正黑體';\" HREF=/newcm.html><b>最新上架</b></A></td>
    <td width=\"148\" background=\"/image/memul.png\"><A class=tmemu style=\"font-size:15px;font-family:'微軟正黑體';\" HREF=https://www.i-gamer.net/web/?ch=album><b>動漫遊戲</b></A></td>
    <td width=\"148\" background=\"/image/memul.png\"><A class=tmemu style=\"font-size:15px;font-family:'微軟正黑體';\" HREF=/collect.html><b>我的收藏</b></A></td>
              </tr>
              <tr>
                <td colspan=\"6\" height=\"12\"></td>
              </tr>
</table>

          </td>
        </tr>

        <tr>
          <td align=\"center\">

<table cellpadding=\"0\" cellspacing=\"0\" border=\"0\" height=\"110\">
  <tr>
    <td background=\"https://img.i-gamer.net/image/cartoon3.png\"><table cellspacing=0 border=0 cellpadding=2><tr align=center valign=bottom height=104><td width=39></td><td width=112 style='font-size:12px;'><table width=104 border=0 cellspacing=1 cellpadding=0 bgcolor=#999999><tr><td bgcolor=#FFFFFF height=77 align=center><a href=https://www.i-gamer.net/site/12592.html target=_blank><img src=https://img.i-gamer.net/gamepic/3/3.1228s.jpg width=100 height=75 border=0></a></td></tr></table>超時空戰士</td><td width=112 style='font-size:12px;'><table width=104 border=0 cellspacing=1 cellpadding=0 bgcolor=#999999><tr><td bgcolor=#FFFFFF height=77 align=center><a href=https://www.i-gamer.net/site/10099.html target=_blank><img src=https://img.i-gamer.net/gamepic/1/1.2229s.jpg width=100 height=75 border=0></a></td></tr></table>進擊的巨人獵手</td><td width=112 style='font-size:12px;'><table width=104 border=0 cellspacing=1 cellpadding=0 bgcolor=#999999><tr><td bgcolor=#FFFFFF height=77 align=center><a href=https://www.i-gamer.net/site/6404.html target=_blank><img src=https://img.i-gamer.net/gamepic/6/6.1371s.jpg width=100 height=75 border=0></a></td></tr></table>完美髮型屋</td><td width=112 style='font-size:12px;'><table width=104 border=0 cellspacing=1 cellpadding=0 bgcolor=#999999><tr><td bgcolor=#FFFFFF height=77 align=center><a href=https://www.i-gamer.net/site/5552.html target=_blank><img src=https://img.i-gamer.net/gamepic/5/5.600s.jpg width=100 height=75 border=0></a></td></tr></table>真‧動漫無雙</td><td width=112 style='font-size:12px;'><table width=104 border=0 cellspacing=1 cellpadding=0 bgcolor=#999999><tr><td bgcolor=#FFFFFF height=77 align=center><a href=https://www.i-gamer.net/site/9243.html target=_blank><img src=https://img.i-gamer.net/gamepic/6/6.1936s.jpg width=100 height=75 border=0></a></td></tr></table>美女變身</td><td width=112 style='font-size:12px;'><table width=104 border=0 cellspacing=1 cellpadding=0 bgcolor=#999999><tr><td bgcolor=#FFFFFF height=77 align=center><a href=https://www.i-gamer.net/site/9293.html target=_blank><img src=https://img.i-gamer.net/gamepic/3/3.882s.jpg width=100 height=75 border=0></a></td></tr></table>戰火英雄２</td><td width=112 style='font-size:12px;'><table width=104 border=0 cellspacing=1 cellpadding=0 bgcolor=#999999><tr><td bgcolor=#FFFFFF height=77 align=center><a href=https://www.i-gamer.net/site/9212.html target=_blank><img src=https://img.i-gamer.net/gamepic/3/3.875s.jpg width=100 height=75 border=0></a></td></tr></table>戰火英雄中文版</td><td width=8></td></tr></table>
</td>
  </tr>
</table>

          </td>
        </tr>
        <tr>
          <td height=\"8\"></td>
        </tr>
        <tr>
          <td align=\"right\">
            <table width=\"890\" border=\"0\" cellspacing=\"0\" cellpadding=\"0\">
              <tr>
                <td width=\"10\"><img src=\"/image/content_box1.gif\" width=\"10\" height=\"29\"></td>
                <td background=\"/image/content_box2.gif\" height=\"29\" valign=\"bottom\" width=\"642\">
                  　 　　<b><font color=\"#FFFFFF\">最 新 上 架 漫 畫</font></b></td>
                <td background=\"/image/content_box2.gif\" height=\"29\" valign=\"bottom\" width=\"66\"><b>&nbsp<a href=\"newcm.html\"><font color=\"#FFFFFF\">&raquo;
                  more</font></a></b></td>
                <td width=\"10\"><img src=\"/image/content_box3.gif\" width=\"10\" height=\"29\"></td>
              </tr>
              <tr>
                <td background=\"/image/content_box4.gif\" width=\"10\">&nbsp;</td>
                <td colspan=\"2\">
                  <table width=\"860\" align=\"center\">
                    <tr>
                    <tr>
                      <td colspan=\"5\" height=\"6\"></td>
                    </tr>
                    <td align=\"center\"><table cellpadding=\"4\"><tr><td><span class=\"covertxt\"><a class=a2 href=comic/61520101.html target=_blank title=前往101話>更新到第 101 話&nbsp; &nbsp;</a></span><a href=comic/6152.html title=\"異世界精靈的奴隸醬\"><span class=\"covers\"></span><img src=\"/cartoonimgs/coimgc/6152c.jpg\" width=\"120\" height=\"160\" border=\"0\"></a></td></tr></table><a class=a1 href=comic/6152.html>異世界精靈的奴隸</a></td><td align=\"center\"><table cellpadding=\"4\"><tr><td><span class=\"covertxt\"><a class=a2 href=comic/57330032.html target=_blank title=前往32話>更新到第 32 話&nbsp; &nbsp;</a></span><a href=comic/5733.html title=\"戰×戀\"><span class=\"covers\"></span><img src=\"/cartoonimgs/coimgc/5733c.jpg\" width=\"120\" height=\"160\" border=\"0\"></a></td></tr></table><a class=a1 href=comic/5733.html>戰×戀</a></td><td align=\"center\"><table cellpadding=\"4\"><tr><td><span class=\"covertxt\"><a class=a2 href=comic/46590023.html target=_blank title=前往23話>更新到第 23 話&nbsp; &nbsp;</a></span><a href=comic/4659.html title=\"異世創生錄 \"><span class=\"covers\"></span><img src=\"/cartoonimgs/coimgc/4659c.jpg\" width=\"120\" height=\"160\" border=\"0\"></a></td></tr></table><a class=a1 href=comic/4659.html>異世創生錄 </a></td><td align=\"center\"><table cellpadding=\"4\"><tr><td><span class=\"covertxt\"><a class=a2 href=comic/46400108.html target=_blank title=前往108話>更新到第 108 話&nbsp; &nbsp;</a></span><a href=comic/4640.html title=\"寄宿學校的朱麗葉 \"><span class=\"covers\"></span><img src=\"/cartoonimgs/coimgc/4640c.jpg\" width=\"120\" height=\"160\" border=\"0\"></a></td></tr></table><a class=a1 href=comic/4640.html>寄宿學校的朱麗葉</a></td><td align=\"center\"><table cellpadding=\"4\"><tr><td><span class=\"covertxt\"><a class=a2 href=comic/42400231.html target=_blank title=前往231話>更新到第 231 話&nbsp; &nbsp;</a></span><a href=comic/4240.html title=\"家有女友\"><span class=\"covers\"></span><img src=\"/cartoonimgs/coimgc/4240c.jpg\" width=\"120\" height=\"160\" border=\"0\"></a></td></tr></table><a class=a1 href=comic/4240.html>家有女友</a></td><td align=\"center\"><table cellpadding=\"4\"><tr><td><span class=\"covertxt\"><a class=a2 href=comic/32960027.html target=_blank title=前往27話>更新到第 27 話&nbsp; &nbsp;</a></span><a href=comic/3296.html title=\"卡片少女召喚脫衣大戰\"><span class=\"covers\"></span><img src=\"/cartoonimgs/coimgc/3296c.jpg\" width=\"120\" height=\"160\" border=\"0\"></a></td></tr></table><a class=a1 href=comic/3296.html>卡片少女召喚脫衣</a></td></tr><tr><td height=6 colspan=5></td></tr><tr><td align=\"center\"><table cellpadding=\"4\"><tr><td><span class=\"covertxt\"><a class=a2 href=comic/17000109.html target=_blank title=前往109話>更新到第 109 話&nbsp; &nbsp;</a></span><a href=comic/1700.html title=\"KARNEVAL狂歡節\"><span class=\"covers\"></span><img src=\"/cartoonimgs/coimgc/1700c.jpg\" width=\"120\" height=\"160\" border=\"0\"></a></td></tr></table><a class=a1 href=comic/1700.html>KARNEVAL</a></td><td align=\"center\"><table cellpadding=\"4\"><tr><td><span class=\"covertxt\"><a class=a2 href=comic/82030006.html target=_blank title=前往6話>更新到第 6 話&nbsp; &nbsp;</a></span><a href=comic/8203.html title=\"狼不會入眠\"><span class=\"covers\"></span><img src=\"/cartoonimgs/coimgc/8203c.jpg\" width=\"120\" height=\"160\" border=\"0\"></a></td></tr></table><a class=a1 href=comic/8203.html>狼不會入眠</a></td><td align=\"center\"><table cellpadding=\"4\"><tr><td><span class=\"covertxt\"><a class=a2 href=comic/81220007.html target=_blank title=前往7話>更新到第 7 話&nbsp; &nbsp;</a></span><a href=comic/8122.html title=\"惡役千金流放後！利用教會改革美食過上悠然的修女生活\"><span class=\"covers\"></span><img src=\"/cartoonimgs/coimgc/8122c.jpg\" width=\"120\" height=\"160\" border=\"0\"></a></td></tr></table><a class=a1 href=comic/8122.html>惡役千金流放後！</a></td><td align=\"center\"><table cellpadding=\"4\"><tr><td><span class=\"covertxt\"><a class=a2 href=comic/79070052.html target=_blank title=前往52話>更新到第 52 話&nbsp; &nbsp;</a></span><a href=comic/7907.html title=\"伊甸星原\"><span class=\"covers\"></span><img src=\"/cartoonimgs/coimgc/7907c.jpg\" width=\"120\" height=\"160\" border=\"0\"></a></td></tr></table><a class=a1 href=comic/7907.html>伊甸星原</a></td><td align=\"center\"><table cellpadding=\"4\"><tr><td><span class=\"covertxt\"><a class=a2 href=comic/78850018.html target=_blank title=前往18話>更新到第 18 話&nbsp; &nbsp;</a></span><a href=comic/7885.html title=\"國八分\"><span class=\"covers\"></span><img src=\"/cartoonimgs/coimgc/7885c.jpg\" width=\"120\" height=\"160\" border=\"0\"></a></td></tr></table><a class=a1 href=comic/7885.html>國八分</a></td><td align=\"center\"><table cellpadding=\"4\"><tr><td><span class=\"covertxt\"><a class=a2 href=comic/77420012.html target=_blank title=前往12話>更新到第 12 話&nbsp; &nbsp;</a></span><a href=comic/7742.html title=\"戀愛雙人組\"><span class=\"covers\"></span><img src=\"/cartoonimgs/coimgc/7742c.jpg\" width=\"120\" height=\"160\" border=\"0\"></a></td></tr></table><a class=a1 href=comic/7742.html>戀愛雙人組</a></td></tr><tr><td height=6 colspan=5></td></tr><tr><td height=13 colspan=5 background=/image/bkline.gif></td></tr><tr><td> ‧<a class=a1 href=comic/7670.html title=\"原最強劍士憧憬著異世界魔法\">原最強劍士憧憬..</a> [<a class=a2 href=comic/76700016.html target=_blank title=更新到第16話><font color=#800000>16</font></a>]</td><td> ‧<a class=a1 href=comic/5441.html title=\"BEASTARS\">BEASTAR..</a> [<a class=a2 href=comic/54410135.html target=_blank title=更新到第135話><font color=#800000>135</font></a>]</td><td> ‧<a class=a1 href=comic/5348.html title=\"兒子可愛過頭的魔族母親\">兒子可愛過頭的..</a> [<a class=a2 href=comic/53480082.html target=_blank title=更新到第82話><font color=#800000>82</font></a>]</td><td> ‧<a class=a1 href=comic/5129.html title=\"青葉同學,請告訴我\">青葉同學,請告..</a> [<a class=a2 href=comic/51290020.html target=_blank title=更新到第20話><font color=#800000>20</font></a>]</td><td> ‧<a class=a1 href=comic/5067.html title=\"藤原同學說的大抵都對\">藤原同學說的大..</a> [<a class=a2 href=comic/50670027.html target=_blank title=更新到第27話><font color=#800000>27</font></a>]</td><td> ‧<a class=a1 href=comic/4613.html title=\"炎炎之消防隊 \">炎炎之消防隊 </a> [<a class=a2 href=comic/46130180.html target=_blank title=更新到第180話><font color=#800000>180</font></a>]</td></tr><tr bgcolor=#EAF3FB height=25><td> ‧<a class=a1 href=comic/2504.html title=\"七原罪\">七原罪</a> [<a class=a2 href=comic/25040318.html target=_blank title=更新到第318話><font color=#800000>318</font></a>]</td><td> ‧<a class=a1 href=comic/2132.html title=\"魔王大人你就收下這個吧\">魔王大人你就收..</a> [<a class=a2 href=comic/21320056.html target=_blank title=更新到第56話><font color=#800000>56</font></a>]</td><td> ‧<a class=a1 href=comic/1427.html title=\"源君物語\">源君物語</a> [<a class=a2 href=comic/14270350.html target=_blank title=更新到第350話><font color=#800000>350</font></a>]</td><td> ‧<a class=a1 href=comic/1387.html title=\"賭博墮天錄 和也篇\">賭博墮天錄 和..</a> [<a class=a2 href=comic/13870323.html target=_blank title=更新到第323話><font color=#800000>323</font></a>]</td><td> ‧<a class=a1 href=comic/8301.html title=\"狩龍人拉格納\">狩龍人拉格納</a> [<a class=a2 href=comic/83010002.html target=_blank title=更新到第2話><font color=#800000>2</font></a>]</td><td> ‧<a class=a1 href=comic/8295.html title=\"黑暗集會\">黑暗集會</a> [<a class=a2 href=comic/82950005.html target=_blank title=更新到第5話><font color=#800000>5</font></a>]</td></tr><tr height=25><td> ‧<a class=a1 href=comic/8270.html title=\"被勇者隊伍開除的馭獸使 邂逅了最強種的貓耳少女\">被勇者隊伍開除..</a> [<a class=a2 href=comic/82700010.html target=_blank title=更新到第10話><font color=#800000>10</font></a>]</td><td> ‧<a class=a1 href=comic/8255.html title=\"沒有名字的怪物\">沒有名字的怪物</a> [<a class=a2 href=comic/82550011.html target=_blank title=更新到第11話><font color=#800000>11</font></a>]</td><td> ‧<a class=a1 href=comic/8200.html title=\"放學後的異世界咖啡館\">放學後的異世界..</a> [<a class=a2 href=comic/82000004.html target=_blank title=更新到第4話><font color=#800000>4</font></a>]</td><td> ‧<a class=a1 href=comic/8186.html title=\"雖然到了異世界但要幹點啥才好呢\">雖然到了異世界..</a> [<a class=a2 href=comic/81860004.html target=_blank title=更新到第4話><font color=#800000>4</font></a>]</td><td> ‧<a class=a1 href=comic/8142.html title=\"登山者與被封印的惡狐小姐\">登山者與被封印..</a> [<a class=a2 href=comic/81420036.html target=_blank title=更新到第36話><font color=#800000>36</font></a>]</td><td> ‧<a class=a1 href=comic/8060.html title=\"Queens Orders\">Queens ..</a> [<a class=a2 href=comic/80600034.html target=_blank title=更新到第34話><font color=#800000>34</font></a>]</td></tr><tr bgcolor=#EAF3FB height=25><td> ‧<a class=a1 href=comic/7989.html title=\"黑鳳蝶\">黑鳳蝶</a> [<a class=a2 href=comic/79890045.html target=_blank title=更新到第45話><font color=#800000>45</font></a>]</td><td> ‧<a class=a1 href=comic/7905.html title=\"影宅\">影宅</a> [<a class=a2 href=comic/79050034.html target=_blank title=更新到第34話><font color=#800000>34</font></a>]</td><td> ‧<a class=a1 href=comic/7798.html title=\"你是勇者吧？請去死吧！\">你是勇者吧？請..</a> [<a class=a2 href=comic/77980008.html target=_blank title=更新到第8話><font color=#800000>8</font></a>]</td><td> ‧<a class=a1 href=comic/6265.html title=\"賢惠幼妻仙狐小姐\">賢惠幼妻仙狐小..</a> [<a class=a2 href=comic/62650037.html target=_blank title=更新到第37話><font color=#800000>37</font></a>]</td><td> ‧<a class=a1 href=comic/6212.html title=\"青之蘆葦\">青之蘆葦</a> [<a class=a2 href=comic/62120183.html target=_blank title=更新到第183話><font color=#800000>183</font></a>]</td><td> ‧<a class=a1 href=comic/6185.html title=\"槍之勇者重生錄\">槍之勇者重生錄</a> [<a class=a2 href=comic/61850022.html target=_blank title=更新到第22話><font color=#800000>22</font></a>]</td></tr><tr height=25><td> ‧<a class=a1 href=comic/5811.html title=\"轉生奇譚\">轉生奇譚</a> [<a class=a2 href=comic/58110017.html target=_blank title=更新到第17話><font color=#800000>17</font></a>]</td><td> ‧<a class=a1 href=comic/5408.html title=\"川柳少女\">川柳少女</a> [<a class=a2 href=comic/54080059.html target=_blank title=更新到第59話><font color=#800000>59</font></a>]</td><td> ‧<a class=a1 href=comic/5336.html title=\"今天也攻略了女孩子\">今天也攻略了女..</a> [<a class=a2 href=comic/53360028.html target=_blank title=更新到第28話><font color=#800000>28</font></a>]</td><td> ‧<a class=a1 href=comic/5242.html title=\"久住同學,會察言觀色嗎\">久住同學,會察..</a> [<a class=a2 href=comic/52420051.html target=_blank title=更新到第51話><font color=#800000>51</font></a>]</td><td> ‧<a class=a1 href=comic/4533.html title=\"請問您今天要來點兔子嗎 \">請問您今天要來..</a> [<a class=a2 href=comic/45330099.html target=_blank title=更新到第99話><font color=#800000>99</font></a>]</td><td> ‧<a class=a1 href=comic/4065.html title=\"錢進球場\">錢進球場</a> [<a class=a2 href=comic/40650129.html target=_blank title=更新到第129話><font color=#800000>129</font></a>]</td></tr><tr bgcolor=#EAF3FB height=25><td> ‧<a class=a1 href=comic/7883.html title=\"Promise·Cinderella\">Promise..</a> [<a class=a2 href=comic/78830040.html target=_blank title=更新到第40話><font color=#800000>40</font></a>]</td><td> ‧<a class=a1 href=comic/7654.html title=\"咒術回戰\">咒術回戰</a> [<a class=a2 href=comic/76540066.html target=_blank title=更新到第66話><font color=#800000>66</font></a>]</td><td> ‧<a class=a1 href=comic/6186.html title=\"群青Reflection\">群青Refle..</a> [<a class=a2 href=comic/61860021.html target=_blank title=更新到第21話><font color=#800000>21</font></a>]</td><td> ‧<a class=a1 href=comic/6059.html title=\"大蜘蛛醬Flashback\">大蜘蛛醬Fla..</a> [<a class=a2 href=comic/60590020.html target=_blank title=更新到第20話><font color=#800000>20</font></a>]</td><td> ‧<a class=a1 href=comic/5490.html title=\"勇者大人洗澡水溫合適嗎\">勇者大人洗澡水..</a> [<a class=a2 href=comic/54900029.html target=_blank title=更新到第29話><font color=#800000>29</font></a>]</td><td> ‧<a class=a1 href=comic/5480.html title=\"我們無法一起學習\">我們無法一起學..</a> [<a class=a2 href=comic/54800118.html target=_blank title=更新到第118話><font color=#800000>118</font></a>]</td>
                    </tr>
                  </table>
                </td>
                <td background=\"/image/content_box5.gif\" width=\"10\">&nbsp;</td>
              </tr>
              <tr>
                <td width=\"10\"><img src=\"/image/content_box6.gif\" width=\"10\" height=\"10\"></td>
                <td background=\"/image/content_box7.gif\" height=\"10\" colspan=\"2\"></td>
                <td width=\"10\"><img src=\"/image/content_box8.gif\" width=\"10\" height=\"10\"></td>
              </tr>
            </table>
          </td>
        </tr>
        <tr>
          <td height=\"10\"></td>
        </tr>
        <tr>
          <td align=\"right\">
            <table width=\"890\" border=\"0\" cellspacing=\"0\" cellpadding=\"0\">
              <tr>
                <td width=\"10\"><img src=\"/image/content_box1.gif\" width=\"10\" height=\"29\"></td>
                <td background=\"/image/content_box2.gif\" height=\"29\" valign=\"bottom\" width=\"642\">
                  　 　　<b><font color=\"#FFFFFF\">網 友 推 荐 漫 畫</font></b></td>
                <td background=\"/image/content_box2.gif\" height=\"29\" valign=\"bottom\" width=\"66\"><b><a href=\"topcm.html\"><font color=\"#FFFFFF\">&raquo;
                  more</font></a></b></td>
                <td width=\"10\"><img src=\"/image/content_box3.gif\" width=\"10\" height=\"29\"></td>
              </tr>
              <tr>
                <td background=\"/image/content_box4.gif\" width=\"10\">&nbsp;</td>
                <td colspan=\"2\">
                  <table width=\"860\" align=\"center\">
                    <tr>
                    <tr>
                      <td colspan=\"5\" height=\"6\"></td>
                    </tr>
                    <td align=\"center\"><table cellpadding=\"4\"><tr><td><span class=\"covertxt\"><a class=a2 href=comic/11520948.html target=_blank title=前往948話>更新到第 948 話&nbsp; &nbsp;</a></span><a href=comic/1152.html title=\"海賊王\"><span class=\"covers\"></span><img src=\"/cartoonimgs/coimgc/1152c.jpg\" width=\"120\" height=\"160\" border=\"0\"></a></td></tr></table><a class=a1 href=comic/1152.html>海賊王</a></td><td align=\"center\"><table cellpadding=\"4\"><tr><td><span class=\"covertxt\"><a class=a2 href=comic/11530545.html target=_blank title=前往545話>更新到第 545 話&nbsp; &nbsp;</a></span><a href=comic/1153.html title=\"魔導少年\"><span class=\"covers\"></span><img src=\"/cartoonimgs/coimgc/1153c.jpg\" width=\"120\" height=\"160\" border=\"0\"></a></td></tr></table><a class=a1 href=comic/1153.html>魔導少年</a></td><td align=\"center\"><table cellpadding=\"4\"><tr><td><span class=\"covertxt\"><a class=a2 href=comic/12210119.html target=_blank title=前往119話>更新到第 119 話&nbsp; &nbsp;</a></span><a href=comic/1221.html title=\"進擊的巨人\"><span class=\"covers\"></span><img src=\"/cartoonimgs/coimgc/1221c.jpg\" width=\"120\" height=\"160\" border=\"0\"></a></td></tr></table><a class=a1 href=comic/1221.html>進擊的巨人</a></td><td align=\"center\"><table cellpadding=\"4\"><tr><td><span class=\"covertxt\"><a class=a2 href=comic/12540409.html target=_blank title=前往409話>更新到第 409 話&nbsp; &nbsp;</a></span><a href=comic/1254.html title=\"家庭教師\"><span class=\"covers\"></span><img src=\"/cartoonimgs/coimgc/1254c.jpg\" width=\"120\" height=\"160\" border=\"0\"></a></td></tr></table><a class=a1 href=comic/1254.html>家庭教師</a></td><td align=\"center\"><table cellpadding=\"4\"><tr><td><span class=\"covertxt\"><a class=a2 href=comic/35830153.html target=_blank title=前往153話>更新到第 153 話&nbsp; &nbsp;</a></span><a href=comic/3583.html title=\"一拳超人\"><span class=\"covers\"></span><img src=\"/cartoonimgs/coimgc/3583c.jpg\" width=\"120\" height=\"160\" border=\"0\"></a></td></tr></table><a class=a1 href=comic/3583.html>一拳超人</a></td><td align=\"center\"><table cellpadding=\"4\"><tr><td><span class=\"covertxt\"><a class=a2 href=comic/10290710.html target=_blank title=前往710話>更新到第 710 話&nbsp; &nbsp;</a></span><a href=comic/1029.html title=\"火影忍者\"><span class=\"covers\"></span><img src=\"/cartoonimgs/coimgc/1029c.jpg\" width=\"120\" height=\"160\" border=\"0\"></a></td></tr></table><a class=a1 href=comic/1029.html>火影忍者</a></td></tr><tr><td height=6 colspan=5></td></tr><tr><td align=\"center\"><table cellpadding=\"4\"><tr><td><span class=\"covertxt\"><a class=a2 href=comic/10840333.html target=_blank title=前往333話>更新到第 333 話&nbsp; &nbsp;</a></span><a href=comic/1084.html title=\"光速蒙面俠21\"><span class=\"covers\"></span><img src=\"/cartoonimgs/coimgc/1084c.jpg\" width=\"120\" height=\"160\" border=\"0\"></a></td></tr></table><a class=a1 href=comic/1084.html>光速蒙面俠21</a></td><td align=\"center\"><table cellpadding=\"4\"><tr><td><span class=\"covertxt\"><a class=a2 href=comic/13000686.html target=_blank title=前往686話>更新到第 686 話&nbsp; &nbsp;</a></span><a href=comic/1300.html title=\"死神\"><span class=\"covers\"></span><img src=\"/cartoonimgs/coimgc/1300c.jpg\" width=\"120\" height=\"160\" border=\"0\"></a></td></tr></table><a class=a1 href=comic/1300.html>死神</a></td><td align=\"center\"><table cellpadding=\"4\"><tr><td><span class=\"covertxt\"><a class=a2 href=comic/12160077.html target=_blank title=前往77話>更新到第 77 話&nbsp; &nbsp;</a></span><a href=comic/1216.html title=\"天降之物\"><span class=\"covers\"></span><img src=\"/cartoonimgs/coimgc/1216c.jpg\" width=\"120\" height=\"160\" border=\"0\"></a></td></tr></table><a class=a1 href=comic/1216.html>天降之物</a></td><td align=\"center\"><table cellpadding=\"4\"><tr><td><span class=\"covertxt\"><a class=a2 href=comic/12570149.html target=_blank title=前往149話>更新到第 149 話&nbsp; &nbsp;</a></span><a href=comic/1257.html title=\"元氣少女緣結神\"><span class=\"covers\"></span><img src=\"/cartoonimgs/coimgc/1257c.jpg\" width=\"120\" height=\"160\" border=\"0\"></a></td></tr></table><a class=a1 href=comic/1257.html>元氣少女緣結神</a></td><td align=\"center\"><table cellpadding=\"4\"><tr><td><span class=\"covertxt\"><a class=a2 href=comic/16980315.html target=_blank title=前往315話>更新到第 315 話&nbsp; &nbsp;</a></span><a href=comic/1698.html title=\"食戟之靈\"><span class=\"covers\"></span><img src=\"/cartoonimgs/coimgc/1698c.jpg\" width=\"120\" height=\"160\" border=\"0\"></a></td></tr></table><a class=a1 href=comic/1698.html>食戟之靈</a></td><td align=\"center\"><table cellpadding=\"4\"><tr><td><span class=\"covertxt\"><a class=a2 href=comic/15940056.html target=_blank title=前往56話>更新到第 56 話&nbsp; &nbsp;</a></span><a href=comic/1594.html title=\"LAST GAME\"><span class=\"covers\"></span><img src=\"/cartoonimgs/coimgc/1594c.jpg\" width=\"120\" height=\"160\" border=\"0\"></a></td></tr></table><a class=a1 href=comic/1594.html>LAST GAM</a></td></tr><tr><td height=6 colspan=5></td></tr><tr><td height=13 colspan=5 background=/image/bkline.gif></td></tr><tr><td> ‧<a class=a1 href=comic/1134.html title=\"魔王的父親\">魔王的父親</a> [<a class=a2 href=comic/11340240.html target=_blank title=更新到第240話><font color=#800000>240</font></a>]</td><td> ‧<a class=a1 href=comic/1082.html title=\"黑子的籃球\">黑子的籃球</a> [<a class=a2 href=comic/10820275.html target=_blank title=更新到第275話><font color=#800000>275</font></a>]</td><td> ‧<a class=a1 href=comic/1155.html title=\"獵人\">獵人</a> [<a class=a2 href=comic/11550390.html target=_blank title=更新到第390話><font color=#800000>390</font></a>]</td><td> ‧<a class=a1 href=comic/1799.html title=\"暗殺教室\">暗殺教室</a> [<a class=a2 href=comic/17990180.html target=_blank title=更新到第180話><font color=#800000>180</font></a>]</td><td> ‧<a class=a1 href=comic/1252.html title=\"會長是女僕大人\">會長是女僕大人</a> [18]</td><td> ‧<a class=a1 href=comic/1416.html title=\"監獄學園\">監獄學園</a> [<a class=a2 href=comic/14160277.html target=_blank title=更新到第277話><font color=#800000>277</font></a>]</td></tr><tr bgcolor=#EAF3FB height=25><td> ‧<a class=a1 href=comic/1285.html title=\"絕對戀愛命令\">絕對戀愛命令</a> [<a class=a2 href=comic/12850074.html target=_blank title=更新到第74話><font color=#800000>74</font></a>]</td><td> ‧<a class=a1 href=comic/1056.html title=\"魔法少年賈修\">魔法少年賈修</a> [33]</td><td> ‧<a class=a1 href=comic/1386.html title=\"白砂糖戰士\">白砂糖戰士</a> [<a class=a2 href=comic/13860047.html target=_blank title=更新到第47話><font color=#800000>47</font></a>]</td><td> ‧<a class=a1 href=comic/1412.html title=\"偽戀\">偽戀</a> [<a class=a2 href=comic/14120230.html target=_blank title=更新到第230話><font color=#800000>230</font></a>]</td><td> ‧<a class=a1 href=comic/1220.html title=\"美食的俘虜\">美食的俘虜</a> [<a class=a2 href=comic/12200396.html target=_blank title=更新到第396話><font color=#800000>396</font></a>]</td><td> ‧<a class=a1 href=comic/1287.html title=\"黑執事\">黑執事</a> [<a class=a2 href=comic/12870153.html target=_blank title=更新到第153話><font color=#800000>153</font></a>]</td></tr><tr height=25><td> ‧<a class=a1 href=comic/1633.html title=\"山田和七個魔女\">山田和七個魔女</a> [<a class=a2 href=comic/16330243.html target=_blank title=更新到第243話><font color=#800000>243</font></a>]</td><td> ‧<a class=a1 href=comic/1066.html title=\"名偵探柯南\">名偵探柯南</a> [<a class=a2 href=comic/10661035.html target=_blank title=更新到第1035話><font color=#800000>1035</font></a>]</td><td> ‧<a class=a1 href=comic/3319.html title=\"翼與螢火蟲\">翼與螢火蟲</a> [<a class=a2 href=comic/33190053.html target=_blank title=更新到第53話><font color=#800000>53</font></a>]</td><td> ‧<a class=a1 href=comic/2030.html title=\"拂曉的尤娜\">拂曉的尤娜</a> [<a class=a2 href=comic/20300175.html target=_blank title=更新到第175話><font color=#800000>175</font></a>]</td><td> ‧<a class=a1 href=comic/1103.html title=\"銀魂\">銀魂</a> [<a class=a2 href=comic/11030704.html target=_blank title=更新到第704話><font color=#800000>704</font></a>]</td><td> ‧<a class=a1 href=comic/2504.html title=\"七原罪\">七原罪</a> [<a class=a2 href=comic/25040318.html target=_blank title=更新到第318話><font color=#800000>318</font></a>]</td></tr><tr bgcolor=#EAF3FB height=25><td> ‧<a class=a1 href=comic/1427.html title=\"源君物語\">源君物語</a> [<a class=a2 href=comic/14270350.html target=_blank title=更新到第350話><font color=#800000>350</font></a>]</td><td> ‧<a class=a1 href=comic/1286.html title=\"華麗的挑戰\">華麗的挑戰</a> [<a class=a2 href=comic/12860269.html target=_blank title=更新到第269話><font color=#800000>269</font></a>]</td><td> ‧<a class=a1 href=comic/1050.html title=\"魔笛MAGI\">魔笛MAGI</a> [<a class=a2 href=comic/10500369.html target=_blank title=更新到第369話><font color=#800000>369</font></a>]</td><td> ‧<a class=a1 href=comic/1643.html title=\"賣肉的灰姑娘\">賣肉的灰姑娘</a> [<a class=a2 href=comic/16430118.html target=_blank title=更新到第118話><font color=#800000>118</font></a>]</td><td> ‧<a class=a1 href=comic/1217.html title=\"殭屍哪有那麼萌?\">殭屍哪有那麼萌..</a> [<a class=a2 href=comic/12170057.html target=_blank title=更新到第57話><font color=#800000>57</font></a>]</td><td> ‧<a class=a1 href=comic/1664.html title=\"蜂蜜初戀\">蜂蜜初戀</a> [<a class=a2 href=comic/16640067.html target=_blank title=更新到第67話><font color=#800000>67</font></a>]</td></tr><tr height=25><td> ‧<a class=a1 href=comic/1798.html title=\"堀與宮村\">堀與宮村</a> [<a class=a2 href=comic/17980092.html target=_blank title=更新到第92話><font color=#800000>92</font></a>]</td><td> ‧<a class=a1 href=comic/1781.html title=\"赤髮白雪姬\">赤髮白雪姬</a> [<a class=a2 href=comic/17810105.html target=_blank title=更新到第105話><font color=#800000>105</font></a>]</td><td> ‧<a class=a1 href=comic/1154.html title=\"妖怪少爺\">妖怪少爺</a> [<a class=a2 href=comic/11540210.html target=_blank title=更新到第210話><font color=#800000>210</font></a>]</td><td> ‧<a class=a1 href=comic/2605.html title=\"貓與我的星期五\">貓與我的星期五</a> [<a class=a2 href=comic/26050065.html target=_blank title=更新到第65話><font color=#800000>65</font></a>]</td><td> ‧<a class=a1 href=comic/2265.html title=\"我的禽獸男友\">我的禽獸男友</a> [<a class=a2 href=comic/22650075.html target=_blank title=更新到第75話><font color=#800000>75</font></a>]</td><td> ‧<a class=a1 href=comic/2803.html title=\"我老婆是學生會長\">我老婆是學生會..</a> [<a class=a2 href=comic/28030069.html target=_blank title=更新到第69話><font color=#800000>69</font></a>]</td></tr><tr bgcolor=#EAF3FB height=25><td> ‧<a class=a1 href=comic/1175.html title=\"特殊傳說\">特殊傳說</a> [7]</td><td> ‧<a class=a1 href=comic/4335.html title=\"CROSS ANGE 天使與龍的學園\">CROSS A..</a> [<a class=a2 href=comic/43350025.html target=_blank title=更新到第25話><font color=#800000>25</font></a>]</td><td> ‧<a class=a1 href=comic/1325.html title=\"雛之戀\">雛之戀</a> [<a class=a2 href=comic/13250061.html target=_blank title=更新到第61話><font color=#800000>61</font></a>]</td><td> ‧<a class=a1 href=comic/1292.html title=\"媽媽是同級生\">媽媽是同級生</a> [<a class=a2 href=comic/12920040.html target=_blank title=更新到第40話><font color=#800000>40</font></a>]</td><td> ‧<a class=a1 href=comic/4105.html title=\"CROSSANGE天使與龍的輪舞\">CROSSAN..</a> [<a class=a2 href=comic/41050043.html target=_blank title=更新到第43話><font color=#800000>43</font></a>]</td><td> ‧<a class=a1 href=comic/1500.html title=\"無法逃離的背叛\">無法逃離的背叛</a> [<a class=a2 href=comic/15000063.html target=_blank title=更新到第63話><font color=#800000>63</font></a>]</td>
                    </tr>
                  </table>
                </td>
                <td background=\"/image/content_box5.gif\" width=\"10\">&nbsp;</td>
              </tr>
              <tr>
                <td width=\"10\"><img src=\"/image/content_box6.gif\" width=\"10\" height=\"10\"></td>
                <td background=\"/image/content_box7.gif\" height=\"10\" colspan=\"2\"></td>
                <td width=\"10\"><img src=\"/image/content_box8.gif\" width=\"10\" height=\"10\"></td>
              </tr>
            </table>
          </td>
        </tr>
        <tr>
          <td height=\"10\"></td>
        </tr>
        <tr>
          <td align=\"right\">
<table width=\"890\" border=\"0\" cellspacing=\"0\" cellpadding=\"0\">
              <tr>
                <td width=\"10\"><img src=\"/image/content_box1.gif\" width=\"10\" height=\"29\"></td>
                <td background=\"/image/content_box2.gif\" height=\"29\" valign=\"bottom\" width=\"642\">
                  　 　　<b><font color=\"#FFFFFF\">經 典 完 結 漫 畫</font></b></td>
                <td background=\"/image/content_box2.gif\" height=\"29\" valign=\"bottom\" width=\"66\"><b><a href=\"endcm.html\"><font color=\"#FFFFFF\">&raquo;
                  more</font></a></b></td>
                <td width=\"10\"><img src=\"/image/content_box3.gif\" width=\"10\" height=\"29\"></td>
              </tr>
              <tr>
                <td background=\"/image/content_box4.gif\" width=\"10\">&nbsp;</td>
                <td colspan=\"2\">
                  <table width=\"860\" align=\"center\">
                    <tr>
                    <tr>
                      <td colspan=\"5\" height=\"6\"></td>
                    </tr>
                    <td align=\"center\"><table cellpadding=\"4\"><tr><td><span class=\"covertxt\"><font color=#FFFF99>共收錄 545 話&nbsp; &nbsp;</font></span><a href=comic/1153.html title=\"魔導少年\"><span class=\"covers\"></span><img src=\"/cartoonimgs/coimgc/1153c.jpg\" width=\"120\" height=\"160\" border=\"0\"></a></td></tr></table><a class=a1 href=comic/1153.html>魔導少年</a></td><td align=\"center\"><table cellpadding=\"4\"><tr><td><span class=\"covertxt\"><font color=#FFFF99>共收錄 710 話&nbsp; &nbsp;</font></span><a href=comic/1029.html title=\"火影忍者\"><span class=\"covers\"></span><img src=\"/cartoonimgs/coimgc/1029c.jpg\" width=\"120\" height=\"160\" border=\"0\"></a></td></tr></table><a class=a1 href=comic/1029.html>火影忍者</a></td><td align=\"center\"><table cellpadding=\"4\"><tr><td><span class=\"covertxt\"><font color=#FFFF99>共收錄 686 話&nbsp; &nbsp;</font></span><a href=comic/1300.html title=\"死神\"><span class=\"covers\"></span><img src=\"/cartoonimgs/coimgc/1300c.jpg\" width=\"120\" height=\"160\" border=\"0\"></a></td></tr></table><a class=a1 href=comic/1300.html>死神</a></td><td align=\"center\"><table cellpadding=\"4\"><tr><td><span class=\"covertxt\"><font color=#FFFF99>共收錄 277 話&nbsp; &nbsp;</font></span><a href=comic/1416.html title=\"監獄學園\"><span class=\"covers\"></span><img src=\"/cartoonimgs/coimgc/1416c.jpg\" width=\"120\" height=\"160\" border=\"0\"></a></td></tr></table><a class=a1 href=comic/1416.html>監獄學園</a></td><td align=\"center\"><table cellpadding=\"4\"><tr><td><span class=\"covertxt\"><font color=#FFFF99>共收錄 180 話&nbsp; &nbsp;</font></span><a href=comic/1799.html title=\"暗殺教室\"><span class=\"covers\"></span><img src=\"/cartoonimgs/coimgc/1799c.jpg\" width=\"120\" height=\"160\" border=\"0\"></a></td></tr></table><a class=a1 href=comic/1799.html>暗殺教室</a></td><td align=\"center\"><table cellpadding=\"4\"><tr><td><span class=\"covertxt\"><font color=#FFFF99>共收錄 179 話&nbsp; &nbsp;</font></span><a href=comic/4270.html title=\"東京食屍鬼RE\"><span class=\"covers\"></span><img src=\"/cartoonimgs/coimgc/4270c.jpg\" width=\"120\" height=\"160\" border=\"0\"></a></td></tr></table><a class=a1 href=comic/4270.html>東京食屍鬼RE</a></td></tr><tr><td height=6 colspan=5></td></tr><tr><td align=\"center\"><table cellpadding=\"4\"><tr><td><span class=\"covertxt\"><font color=#FFFF99>共收錄 396 話&nbsp; &nbsp;</font></span><a href=comic/1220.html title=\"美食的俘虜\"><span class=\"covers\"></span><img src=\"/cartoonimgs/coimgc/1220c.jpg\" width=\"120\" height=\"160\" border=\"0\"></a></td></tr></table><a class=a1 href=comic/1220.html>美食的俘虜</a></td><td align=\"center\"><table cellpadding=\"4\"><tr><td><span class=\"covertxt\"><font color=#FFFF99>共收錄 118 話&nbsp; &nbsp;</font></span><a href=comic/1643.html title=\"賣肉的灰姑娘\"><span class=\"covers\"></span><img src=\"/cartoonimgs/coimgc/1643c.jpg\" width=\"120\" height=\"160\" border=\"0\"></a></td></tr></table><a class=a1 href=comic/1643.html>賣肉的灰姑娘</a></td><td align=\"center\"><table cellpadding=\"4\"><tr><td><span class=\"covertxt\"><font color=#FFFF99>共收錄 583 話&nbsp; &nbsp;</font></span><a href=comic/1031.html title=\"史上最強弟子兼一\"><span class=\"covers\"></span><img src=\"/cartoonimgs/coimgc/1031c.jpg\" width=\"120\" height=\"160\" border=\"0\"></a></td></tr></table><a class=a1 href=comic/1031.html>史上最強弟子兼一</a></td><td align=\"center\"><table cellpadding=\"4\"><tr><td><span class=\"covertxt\"><font color=#FFFF99>共收錄 149 話&nbsp; &nbsp;</font></span><a href=comic/1257.html title=\"元氣少女緣結神\"><span class=\"covers\"></span><img src=\"/cartoonimgs/coimgc/1257c.jpg\" width=\"120\" height=\"160\" border=\"0\"></a></td></tr></table><a class=a1 href=comic/1257.html>元氣少女緣結神</a></td><td align=\"center\"><table cellpadding=\"4\"><tr><td><span class=\"covertxt\"><font color=#FFFF99>共收錄 230 話&nbsp; &nbsp;</font></span><a href=comic/1412.html title=\"偽戀\"><span class=\"covers\"></span><img src=\"/cartoonimgs/coimgc/1412c.jpg\" width=\"120\" height=\"160\" border=\"0\"></a></td></tr></table><a class=a1 href=comic/1412.html>偽戀</a></td><td align=\"center\"><table cellpadding=\"4\"><tr><td><span class=\"covertxt\"><font color=#FFFF99>共收錄 275 話&nbsp; &nbsp;</font></span><a href=comic/1082.html title=\"黑子的籃球\"><span class=\"covers\"></span><img src=\"/cartoonimgs/coimgc/1082c.jpg\" width=\"120\" height=\"160\" border=\"0\"></a></td></tr></table><a class=a1 href=comic/1082.html>黑子的籃球</a></td></tr><tr><td height=6 colspan=5></td></tr><tr><td height=13 colspan=5 background=/image/bkline.gif></td></tr><tr><td> ‧<a class=a1 href=comic/1284.html title=\"最近我的妹妹有點怪\">最近我的妹妹有..</a></td><td> ‧<a class=a1 href=comic/1050.html title=\"魔笛MAGI\">魔笛MAGI</a></td><td> ‧<a class=a1 href=comic/1254.html title=\"家庭教師\">家庭教師</a></td><td> ‧<a class=a1 href=comic/1134.html title=\"魔王的父親\">魔王的父親</a></td><td> ‧<a class=a1 href=comic/1313.html title=\"驚爆遊戲\">驚爆遊戲</a></td><td> ‧<a class=a1 href=comic/1285.html title=\"絕對戀愛命令\">絕對戀愛命令</a></td></tr><tr bgcolor=#EAF3FB height=25><td> ‧<a class=a1 href=comic/2650.html title=\"東京食屍鬼\">東京食屍鬼</a></td><td> ‧<a class=a1 href=comic/1594.html title=\"LAST GAME\">LAST GA..</a></td><td> ‧<a class=a1 href=comic/1252.html title=\"會長是女僕大人\">會長是女僕大人</a></td><td> ‧<a class=a1 href=comic/1386.html title=\"白砂糖戰士\">白砂糖戰士</a></td><td> ‧<a class=a1 href=comic/3319.html title=\"翼與螢火蟲\">翼與螢火蟲</a></td><td> ‧<a class=a1 href=comic/1952.html title=\"火星異種\">火星異種</a></td></tr><tr height=25><td> ‧<a class=a1 href=comic/1569.html title=\"鐵將縱橫2012\">鐵將縱橫201..</a></td><td> ‧<a class=a1 href=comic/2655.html title=\"誠如神之所說Ⅱ\">誠如神之所說Ⅱ</a></td><td> ‧<a class=a1 href=comic/3283.html title=\"戀著的她X愛著的她\">戀著的她X愛著..</a></td><td> ‧<a class=a1 href=comic/4389.html title=\"拳願阿修羅 \">拳願阿修羅 </a></td><td> ‧<a class=a1 href=comic/1539.html title=\"國王遊戲\">國王遊戲</a></td><td> ‧<a class=a1 href=comic/1633.html title=\"山田和七個魔女\">山田和七個魔女</a></td></tr><tr bgcolor=#EAF3FB height=25><td> ‧<a class=a1 href=comic/2416.html title=\"香蕉要進入姐姐身體麼?\">香蕉要進入姐姐..</a></td><td> ‧<a class=a1 href=comic/1166.html title=\"斬·赤紅之瞳\">斬·赤紅之瞳</a></td><td> ‧<a class=a1 href=comic/1295.html title=\"思春期誘惑\">思春期誘惑</a></td><td> ‧<a class=a1 href=comic/1664.html title=\"蜂蜜初戀\">蜂蜜初戀</a></td><td> ‧<a class=a1 href=comic/1216.html title=\"天降之物\">天降之物</a></td><td> ‧<a class=a1 href=comic/1854.html title=\"極黑的布倫希爾特\">極黑的布倫希爾..</a></td></tr><tr height=25><td> ‧<a class=a1 href=comic/1181.html title=\"獸性之愛\">獸性之愛</a></td><td> ‧<a class=a1 href=comic/2605.html title=\"貓與我的星期五\">貓與我的星期五</a></td><td> ‧<a class=a1 href=comic/1154.html title=\"妖怪少爺\">妖怪少爺</a></td><td> ‧<a class=a1 href=comic/1255.html title=\"今天開始戀愛吧\">今天開始戀愛吧</a></td><td> ‧<a class=a1 href=comic/2787.html title=\"到我家來吧\">到我家來吧</a></td><td> ‧<a class=a1 href=comic/1217.html title=\"殭屍哪有那麼萌?\">殭屍哪有那麼萌..</a></td></tr><tr bgcolor=#EAF3FB height=25><td> ‧<a class=a1 href=comic/1325.html title=\"雛之戀\">雛之戀</a></td><td> ‧<a class=a1 href=comic/1453.html title=\"狼少女與黑王子\">狼少女與黑王子</a></td><td> ‧<a class=a1 href=comic/1282.html title=\"OO學院XX科\">OO學院XX科</a></td><td> ‧<a class=a1 href=comic/1948.html title=\"鑽石王牌\">鑽石王牌</a></td><td> ‧<a class=a1 href=comic/1201.html title=\"烏龍派出所\">烏龍派出所</a></td><td> ‧<a class=a1 href=comic/2274.html title=\"浪漫時鐘\">浪漫時鐘</a></td>
                    </tr>
                  </table>
                </td>
                <td background=\"/image/content_box5.gif\" width=\"10\">&nbsp;</td>
              </tr>
              <tr>
                <td width=\"10\"><img src=\"/image/content_box6.gif\" width=\"10\" height=\"10\"></td>
                <td background=\"/image/content_box7.gif\" height=\"10\" colspan=\"2\"></td>
                <td width=\"10\"><img src=\"/image/content_box8.gif\" width=\"10\" height=\"10\"></td>
              </tr>
            </table>
          	</td>
        </tr>

        <tr>
          <td height=\"10\"></td>
        </tr>
      </table>
    </td>
  </tr>
  <tr>
    <td colspan=\"2\">
      <center>
  <br>
  <table width=\"600\" border=\"0\">
    <tr>
      <td align=\"center\">｜ <a href=\"/about.html\">關於動漫狂</a> ｜<a href=\"/private.html\">隱私權政策</a>
        ｜<a href=\"/rule.html\"> 服務條款</a> ｜ <a href=\"/exempt.html\">免責聲明</a> ｜ <a href=\"mailto:service@cartoonmad.com\">策略合作</a>
        ｜<a href=\"mailto:service@cartoonmad.com\"> 與我們聯絡</a> ｜ </td>
    </tr>
    <tr>
      <td align=\"center\">Copyright By www.CarToonMad.Com All Rights Reserve</td>
    </tr>
  </table>
  <br>
</center>
<script type=\"text/javascript\">

  var _gaq = _gaq || [];
  _gaq.push(['_setAccount', 'UA-9323502-2']);
  _gaq.push(['_trackPageview']);

  (function() {
    var ga = document.createElement('script'); ga.type = 'text/javascript'; ga.async = true;
    ga.src = ('https:' == document.location.protocol ? 'https://ssl' : 'http://www') + '.google-analytics.com/ga.js';
    var s = document.getElementsByTagName('script')[0]; s.parentNode.insertBefore(ga, s);
  })();

</script>



    </td>
  </tr>
</table>
</body>
</html>
  ";

  let complete_html_text = format!("<!DOCTYPE html>{}", html_text);

  // Then parse it to dom tree
  let document = Html::parse_document(&complete_html_text);

  // Render the template
  Template::render("tests/dmk/fetch_dmk_home", FetchDmkHomeTemplateData {
    text: &complete_html_text
  })
}
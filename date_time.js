/* JavaScript code from https://learning.oreilly.com/library/view/sams-teach-yourself/9780135167069/ch04.xhtml */

now = new Date();
localtime = now.toString();
utctime = now.toGMTString();
document.write("<p><strong>Local time:</strong> " + localtime + "</p>");
document.write("<p><strong>UTC time:</strong> " + utctime + "</p>");

hours = now.getHours();
mins = now.getMinutes();
secs = now.getSeconds();
document.write("<p><strong>");
document.write(hours + ":" + mins + ":" + secs);
document.write("</p></strong>");
/* document.write("</h2>"; /* deliberately provokes an error!  */

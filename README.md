In main: 
Citim din linia de comanda argumentele folosind functia "std::env::args", pentru ca mai apoi sa transformam ce s-a citit intr-un vector de string-uri folosind functia "collect()", careia ii specificam tipul prin "args()".
Mai apoi, verificam pe rand al primul argument(stiind ca argumentul 0 este numele programului):

Pentru "pwd()": 
	folosim "std::env::current_dir()" pentru a determina directorul curent. Daca acesta nu exista, se returneaza eroare si se afiseaza "Could not find path!". In schimb, daca aceasta exista, folosim functia "to_str()" pentru a transforma buffer-ul oferit de "current_dir()" intr-un "&string". Daca totul este okay, afisam calea.

Pentru "echo(message)":
	daca se apeleaza cu "-n":
		daca avem un singur cuvant, se apeleaza direct echo(cuvant), altfel folosim functia "join(" ")" pentru argumentele de la cel curent pana la final si abia apoi apelam functia care, folosindu-se de "trim()", sterge "\n" de la finalul argumentului dat ca parametru si il afiseaza.
	daca se apeleaza fara "-n":
		se intampla acelasi lucru doar ca se mai printeaza un "\n" 

Pentru "cat(nume_fisiere)":
	daca se apeleaza fara argumente, returneaza eroare.
	altfel, se citeste continutul fiecarui fisier intr-un string, care mai apoi este printat.


Pentru "mkdir(nume_directoare)":
	daca se apeleaza fara argumente, returneaza eroare.
	altfel, se verifica daca exista fisierul si daca exista, nu se face nimic, altfel, este creat.


Pentru "mv(sursa, destinatie)":
	redenumeste sursa cu numele destinatiei.


Pentru "ln([optiune] sursa nume_link)":
	daca primeste mai putin de doua argumente, returneaza eroare.
	altfel, daca are 3 argumente si primeste "-s", foloseste symlink si face un link simbolic.
	altfel, daca are doar doua argumente, face hardlink intre sursa si nume.
	orice alt caz returneaza eroare.

Pentru "rmdir(directoare)":
	pentru fiecare director din lista data, se verifica daca este gol, iar daca nu, returneaza eroare, altfel, acesta este sters.


Pentru "rm([options] fichiere/directoare)":
	dacă comanda începe cu "-r", "-R" sau "--recursive", se va itera prin argumentele rămase și se va încerca să șteargă directoarele și fișierele folosind fs::remove_dir_all() sau fs::remove_file(). Dacă apare o eroare în timpul ștergerii, funcția returnează o eroare cu codul -70; altfel, returnează succes cu codul 0.
	dacă comanda începe cu "-d" sau "--dir", funcția va apela o altă funcție "rmdir" pentru a șterge directoarele specificate. Dacă operația este realizată cu succes, funcția returnează succes cu codul 0; altfel, returnează o eroare cu codul -70.
	in oricare alt caz, funcția va itera prin toate argumentele și va încerca să șteargă fișierele. Dacă apare o eroare în timpul ștergerii, funcția returnează o eroare cu codul -70; altfel, returnează succes cu codul 0.


Pentru "ls ([options] [director])":
	Afișează conținutul directorului specificat sau al directorului curent, bazat pe argumentele primite de la linia de comandă.
	Suportă afișarea fișierelor și a directorilor, inclusiv fișierele ascunse(daca exista -a).
	Realizează afișarea recursivă a conținutului directorului(daca se foloseste -r), folosind funcția "ls_recursive" care :
		Primește un director de bază și o variabila bool pentru fișiere ascunse.
		Afișează conținutul directorului și eventual a altor directoare în mod recursiv.
		Ignoră fișierele ascunse (daca variabila pentru fisiere ascunse este falsa)
		Returnează o eroare cu codul -80 în cazul unor erori.
	Returnează o eroare cu codul -80 în cazul unor erori.
	
	

Pentru "cp" :
	Primește calea către sursa și destinația dorite, precum și un indicator pentru copiere recursivă. (Primește un director sursă și un director destinație.
Verifică dacă sursa este un fișier și destinația este un director, apoi copiază fișierul sursă în destinație.
Dacă sursa este un director și destinația nu există, creează directorul destinație și copiază recursiv conținutul directorului sursă în destinație.
Dacă sursa este un director și destinația există, creează un director în destinație și copiază recursiv conținutul directorului sursă în noul director din destinație.
În cazul unor erori în timpul copierii, funcția returnează o eroare cu codul -90. Altfel, returnează succes.)
	Dacă copierea recursivă este specificată și destinația nu este goală, copiază recursiv conținutul sursa în destinația.
	Dacă copierea recursivă este specificată și destinația este goală, copiază recursiv conținutul sursa în directorul curent.
	Dacă copierea nu este recursivă, copiază fișierul sursă în destinație, fie într-un fișier cu același nume, fie într-un director nou cu același nume ca sursa.
	În cazul unor erori în timpul copierii, funcția returnează o eroare cu codul -90. Altfel, returnează succes.


Pentru "touch":
	Dacă primul argument este "-a":

Dacă al doilea argument este "-c" sau "--no-create":
Dacă fișierul specificat există, îl deschide și verifică conținutul.
În cazul în care există, funcția returnează succes, altfel returnează o eroare cu codul -100.
Dacă al doilea argument nu este "-c" sau "--no-create":
Dacă fișierul specificat există, îl deschide și verifică conținutul.
În cazul în care nu există, funcția creează fișierul.
Dacă fișierul există, funcția verifică conținutul.
Dacă totul se desfășoară cu succes, funcția returnează succes; în caz contrar, returnează o eroare cu codul -100.
Dacă primul argument este "-m":

Dacă al doilea argument este "-c" sau "--no-create":
Dacă fișierul specificat există, îl creează și scrie un caracter în el.
În cazul în care nu există, funcția returnează succes.
Dacă al doilea argument nu este "-c" sau "--no-create":
Dacă fișierul specificat există, îl creează și scrie un spațiu în el.
Dacă fișierul nu există, funcția creează fișierul.
În cazul în care totul se desfășoară cu succes, funcția returnează succes; în caz contrar, returnează o eroare cu codul -100.
Dacă primul argument este "-c" sau "--no-create":

Dacă fișierul specificat există, funcția îl deschide și îl suprascrie cu conținutul sa.
Dacă fișierul nu există, funcția returnează succes.
Dacă operațiunile se desfășoară cu succes, funcția returnează succes; în caz contrar, returnează o eroare cu codul -100.
În oricare alt caz:

Dacă fișierul specificat există, funcția îl deschide, îl șterge și creează un fișier nou cu conținutul "a".
Dacă fișierul nu există, funcția creează fișierul cu conținutul "a".
Dacă operațiunile se desfășoară cu succes, funcția returnează succes; în caz contrar, returnează o eroare cu codul -100.

Pentru "cp":
Funcția acceptă șiruri de caractere "mode" sub forma "+rwx", "-rwx" sau "=rwx", unde "+", "-", sau "=" specifică dacă se adaugă, se elimină sau se setează permisiunile, iar "rwx" specifică drepturile.
Funcția identifică operatorul de modificare ("+", "-", "=") și permisiunile dorite ("r", "w", "x") din șirul "mode".
Dacă se specifică "a", toate categoriile de utilizatori (utilizator, grup și alți utilizatori) sunt luate în considerare.
Funcția aplică modificările asupra permisiunilor fișierului țintă în funcție de operator și permisiuni.
Dacă șirul "mode" are forma octală (de exemplu, "0755"), funcția setează permisiunile fișierului la valorile specificate în modul octal.
Dacă șirul "mode" nu respectă niciuna dintre formatele acceptate, funcția încearcă să identifice utilizatorii ("u", "g", "o") și drepturile ("r", "w", "x") și să aplice modificările corespunzătoare.
Funcția returnează o eroare cu codul -25 în caz de eșec la modificarea permisiunilor sau returnează succes după aplicarea cu succes a modificărilor.


(pe local imi ia toate testele de chmod, dar pe autograding nu imi ia unul)

	




	


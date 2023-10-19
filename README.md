# shut_the_box
Einleitung:

Im folgenden wollen wir das Würfelspiel "Shut the Box" analysieren und herausfinden, was mathematisch gesehen, die beste Herangehensweise and das Würfelspiel ist. 
Dafür werden wir das Spiel mit Hilfe der Stochastik beleuchten und außerdem verschiedene Rust-Anwendunen schreiben, die uns Helfen sollen, die beste Spielweise herauszufinden.

Ziele dieser Arbeit:

Usner Ziel ist herauszufinden, was die beste Heuristik ist, um das Spiel zu gewinnen (und dass unsere Variante die besser ist). Dafür haben wir unsere Analyse und Experimente wie folgt aufgebaut:

-Was ist "Shut the Box"
-Varianten:
	-1. Variante "Penalty score"
	-2. Variante "Eigene Varinate"




Was ist "Shut the Box":

Shut the Box ist ein traditionelles Würfelspiel, das seinen Ursprung im 12. Jahrhundert in Frankreich hat. Es wurde von Seeleuten als Unterhaltungsspiel genutzt und verbreitete sich über die Jahre in verschiedenen Kulturen. Das Spiel erhielt seinen Namen, weil die Spieler versuchen, alle Klappen oder "Boxen" auf einem Spielbrett zu schließen, indem sie Würfeln und die Summe der geworfenen Augenzahlen verwenden. Shut the Box wurde im Laufe der Zeit zu einem beliebten Pub-Spiel und ist heute in verschiedenen Varianten auf der ganzen Welt bekannt. Es bietet eine einfache, aber unterhaltsame Möglichkeit, Glück und Strategie zu kombinieren.

Varianten:

Es gibt verschiedene Variante dieses Spiels, jedoch behandeln wir im folgenden nur die weitläufigeste und die Variante, die wir auch selber Spielen, da diese der Grund war, warum dieses Dokument existiert

1. Variante "Penalty score":

Eine Partie besteht aus mehreren Runden. Jeder Spieler kommt einmal in jeder Runde an die Reihe. Ist ein Spieler am Zuge, werden zuerst alle Klappen geöffnet. Dann beginnt er sein Spiel mit zwei Würfeln und schließt die entsprechenden Klappen.

Für das Schließen der Klappen gilt die Regel, dass jede Zerlegung der Augensumme genutzt werden darf. D. h. zeigen die beiden Würfel z. B. eine 6 und eine 3, so kann er eine beliebige Zerlegung der Augensumme 9, also 9, 1+8, 2+7, 3+6, 4+5, 1+2+6, 1+3+5, ... wählen und die Klappen mit den entsprechenden Nummern schließen.

Sobald die Summe der noch offenen Klappen sechs oder weniger ergibt, setzt der Spieler seinen Zug mit nur einem statt mit beiden Würfeln fort. Der Zug endet, sobald keine Klappe mehr geschlossen werden kann.

Nun wird seine (Minus-)Punktezahl, sein Penalty score, ermittelt, dazu werden die Nummern der nicht geschlossenen Klappen addiert, d. h. sind die Klappen mit den Nummern 1, 5 und 9 offen geblieben, so zählt dies 1 + 5 + 9 = 15 Punkte. Danach ist der nächste Spieler am Wurf.

Das Ziel eines jeden Spielers ist es, alle Klappen zu schließen (Shut the box) und keine Minuspunkte zu schreiben.

Es werden so viele Runden gespielt, bis alle Spieler außer einem einen Penalty score von 45 oder mehr Punkten erreicht haben (45 ist die Summe aller Klappen, denn 1 + 2 + ... + 9 = 45).

Die Minuspunkte werden dabei laufend addiert, sodass die einzelnen Spieler nacheinander ausscheiden und der letzte Überlebende gewinnt.


Bsp: Würfel ich eine 1 und eine 6 und alle Klappen sind geöffnet, so könnte ich die Klappe 7 schließen, die Klappen 1 und 6 schließen, die Klappen 2 und 5 schließen oder die Klappen 3 und 4 schließen.


Quelle: https://de.wikipedia.org/wiki/Shut_the_Box#Varianten


2. Variante "Eigene Variante":

Bei der eigenen Variante besteht ein Partie auch aus mehreren Runden.  Jeder Spieler kommt einmal in jeder Runde an die Reihe und ist ein Spieler neu am Zuge, werden zuerst alle Klappen geöffnet. Der Spieler versucht wie bei der 1. Variante alle Klappen zu schließen, dabei hat er auch zwei Würfel, die er auf einmal wirft. Bei jedem Wurf muss der Spieler mindestens eine Klappe schließen, sonst ist die Runde vorbei und die Zahlen, die auf den noch geöffneten Klappen stehen, werden zusammenaddiert und sind der Punktestand für den Spieler in dieser Runde. Der Spieler mit den wenigsten Punkten gewinnt die Runde und darf deshalb die nächste Runde beginnen. Der Hauptunterschie zur 1. Variante ist, wie die Würfel gewertet und abgearbeite werden können. Die gewürfelten Augenzahlen können entweder zusammen addiert werden, um die Klappen mit den Zahlen 7, 8 und 9 zu schließen oder einzeln verwertet. Zusätzlich könnte man die gewürfelten Augenzahlen auch seperat verwerten, dabei kann man jedoch im Gegensatz zur 1. Variante beide oder nur einen Würferl verwenden. 

Bsp.: Würfel ich eine 1 und eine 6 und alle Klappen sind geöffnet, so könnte ich die Klappe 7 schließen, die Klappen 1 und 6 schließen, die Klappe 6 schließen oder die Klappen 1 schließen. 
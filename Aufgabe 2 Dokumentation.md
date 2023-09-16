# St. Egano

Dokumentation - Jonte

## Erklärung des Programmes

`def rgb_of_pixel(x, y)` ist die Funktion, um die RGB Werte eines bestimmten Pixels zu bekommen.
Dabei werden die Informationen des Pixels inerhalb eines Dictionarys gespeichert.

Die Funktion `def next_pixel(pixel)` berechnet den nächsten Pixel nach dem aktuellen anhand des Grün- und Blauwertes.
Hierbei wird beachtet, dass der offset sich immer auf dem Bild befinden muss. Dies mache ich mit dem Modulo
Operator (%).

Die Funktion `def extract_keyword()` extrahiert das Keyword aus dem Bild. Dabei wird der erste Pixel des Bildes genommen
und dann mit der Funktion `def next_pixel(pixel)` der nächste Pixel berechnet. Dies wird solange wiederholt, bis der
End-Pixel erreicht ist. Der Rote Wert des Pixels wird dann in einen Buchstaben umgewandelt und an den String des
Keywordes angehängt.

Die Funktion `def extract_keyword()` wird dann 7 mal aufgerufen mithalb eines for-Loops. Dies wird deswegen gemacht,
da es 7 Bilder gibt.

## Ergebnisse

### 1. Bild

```txt
Hallo Welt
```

### 2. Bild

```txt
Hallo Gloria

Wie treffen uns am Freitag um 15:00 Uhr vor der Eisdiele am Markplatz.

Alle Liebe,
Juliane
```

### 3. Bild

```txt
Hallo Juliane,

Super, ich werde da sein! Ich freue mich schon auf den riesen Eisbecher mit Erdbeeren.

Bis bald,
Gloria
```

### 4. Bild

```txt
Der Jugendwettbewerb Informatik ist ein Programmierwettbewerb für alle, die erste Programmiererfahrungen sammeln und vertiefen möchten. Programmiert wird mit Blockly, einer Bausteinorientierten Programmiersprache. Vorkenntnisse sind nicht nötig. Um sich mit den Aufgaben des Wettbewerbs vertraut zu machen, empfehlen wir unsere Trainingsseite . Er richtet sich an Schülerinnen und Schüler der Jahrgangsstufen 5 - 13, prinzipiell ist aber eine Teilnahme ab Jahrgangsstufe 3 möglich. Der Wettbewerb besteht aus drei Runden. Die ersten beiden Runden erfolgen online. In der 3. Runde werden zwei Aufgaben gestellt, diese gilt es mit eigenen Programmierwerkzeugen zuhause zu bearbeiten.
```

### 5. Bild

```txt
Der Bundeswettbewerb Informatik richtet sich an Jugendliche bis 21 Jahre, vor dem Studium oder einer Berufstätigkeit. Der Wettbewerb beginnt am 1. September, dauert etwa ein Jahr und besteht aus drei Runden. Dabei können die Aufgaben der 1. Runde ohne größere Informatikkenntnisse gelöst werden; die Aufgaben der 2. Runde sind deutlich schwieriger.

Der Bundeswettbewerb ist fachlich so anspruchsvoll, dass die Gewinner i.d.R. in die Studienstiftung des deutschen Volkes aufgenommen werden. Aus den Besten werden die TeilnehmerInnen für die Internationale Informatik-Olympiade ermittelt. Der Bundeswettbewerb ermöglicht den Teilnehmenden, ihr Wissen zu vertiefen und ihre Begabung weiterzuentwickeln. So trägt der Wettbewerb dazu bei, Jugendliche mit besonderem fachlichen Potenzial zu erkennen.
```

### 6. Bild

```txt
Bonn

Die Bundesstadt Bonn (im Latein der Humanisten Bonna) ist eine kreisfreie Großstadt im Regierungsbezirk Köln im Süden des Landes Nordrhein-Westfalen und Zweitregierungssitz der Bundesrepublik Deutschland. Mit 336.465 Einwohnern (31. Dezember 2022) zählt Bonn zu den zwanzig größten Städten Deutschlands. Bonn gehört zu den Metropolregionen Rheinland und Rhein-Ruhr sowie zur Region Köln/Bonn. Die Stadt an beiden Ufern des Rheins war von 1949 bis 1973 provisorischer Regierungssitz und von 1973 bis 1990 Bundeshauptstadt und bis 1999 Regierungssitz Deutschlands, danach wurde sie zweiter Regierungssitz. Die Vereinten Nationen unterhalten seit 1951 hier einen Sitz.

Bonn kann auf eine mehr als 2000-jährige Geschichte zurückblicken, die auf germanische und römische Siedlungen zurückgeht, und ist damit eine der ältesten Städte Deutschlands. Von 1597 bis 1794 war es Haupt- und Residenzstadt des Kurfürstentums Köln. 1770 kam Ludwig van Beethoven hier zur Welt. Im Laufe des 19. Jahrhunderts entwickelte sich die 1818 gegründete Universität Bonn zu einer der bedeutendsten deutschen Hochschulen.

1948/49 tagte in Bonn der Parlamentarische Rat und arbeitete das Grundgesetz für die Bundesrepublik Deutschland aus, deren erster Parlaments- und Regierungssitz Bonn 1949 wurde. In der Folge erfuhr die Stadt eine umfangreiche Erweiterung und wuchs über das neue Parlaments- und Regierungsviertel mit Bad Godesberg zusammen. Daraus resultierte die Neubildung der Stadt durch Zusammenschluss der Städte Bonn, Bad Godesberg, des rechtsrheinischen Beuel und Gemeinden des vormaligen Landkreises Bonn am 1. August 1969.

Nach der Wiedervereinigung 1990 fasste der Bundestag 1991 den Bonn/Berlin-Beschluss, infolge dessen der Parlaments- und Regierungssitz 1999/2000 in die Bundeshauptstadt Berlin und im Gegenzug zahlreiche Bundesbehörden nach Bonn verlegt wurden. Seitdem haben in der Bundesstadt der Bundespräsident, der Bundeskanzler und der Bundesrat einen zweiten Dienstsitz, gemäß dem Berlin/Bonn-Gesetz sechs Bundesministerien ihren ersten Dienstsitz, die anderen acht einen Zweitsitz. Mit dem Namenszusatz Bundesstadt stärkt der Bund den Standort Bonn als Zweitregierungssitz.

Bonn weist als Sitz von 20 Organisationen der Vereinten Nationen (UN) einen hohen Grad internationaler Verflechtung auf, trägt den Titel UN-Stadt und wird häufig als Welthauptstadt für Nachhaltigkeit und Klimaschutz bezeichnet. Zudem sind die beiden DAX-Unternehmen Deutsche Post und Deutsche Telekom gesetzlich hier ansässig.

Besonders wegen der Sitze von Organisationen und Unternehmen wird das Stadtbild neben Kirchtürmen durch mehrere Hochhäuser geprägt. Dies unterstreicht auch die Bedeutung als Büro-Immobilienmarkt mit mehr als vier Millionen Quadratmetern Fläche.

Geographie

Topografie und Landschaftsräume

Am Übergang vom Mittelrheingebiet zur Niederrheinischen Bucht, der durch den Godesberger Rheintaltrichter markiert wird, liegt im Südwesten des Landes Nordrhein-Westfalen die Stadt Bonn. Auf 141,1 Quadratkilometer dehnt sie sich zu beiden Seiten des Rheines aus. Dabei bilden die linksrheinischen Stadtteile etwa drei Viertel der Gesamtfläche.

Im Süden und Westen umschließen die Ausläufer der Eifel mit dem zum Naturpark Rheinland gehörenden Kottenforst und die Voreifel die Stadt. Nördlich von Bonn öffnet sich das Rheintal in die ab dem Ortsteil Duisdorf bergseitig zur Ville hin vom Vorgebirge begleitete Kölner Bucht. Die hier mündende Sieg stellt im Nordosten die natürliche Grenze dar, das Siebengebirge im Südosten, während im Osten noch einige rechtsrheinische Ortsteile im Pleiser Hügelland liegen. Jenseits des Siebengebirges erstreckt sich südöstlich von Bonn der Westerwald, jenseits der Siegniederung nordöstlich das Bergische Land.

Bonn hat seinen geografischen Mittelpunkt am Bundeskanzlerplatz, der sich im Ortsteil Gronau befindet. Die geografische Lage des Platzes ist 50° 43' 8,8'' N, 7° 7'' 3,3'' O. Die Bonner Innenstadt, die nicht zum Rhein hin ausgerichtet ist, liegt auf einer Höhe von 56 m bis 61 m ü. NHN.

Die größte Ausdehnung des Stadtgebiets in Nord-Süd-Richtung beträgt 15 Kilometer, in West-Ost-Richtung 12,5 Kilometer. Die Stadtgrenzen haben eine Länge von 61 Kilometer.

Auf der rechten Rheinseite liegt der Ennert, der nördliche Ausläufer des Siebengebirges, auf dem Bonner Stadtgebiet. Zu ihm gehört der Paffelsberg, der mit 195,3 m ü. NHN als die höchste Erhebung der Stadt Bonn gilt. Weitere Erhebungen in diesem Höhenzug sind der namensgebende Ennert, Holtorfer Hardt und Röckesberg sowie jeweils mit markanten Steilhängen Rabenlay und Kuckstein, westlich vorgelagert ist der Finkenberg. Auf der linken Flussseite sind die dominierenden Erhebungen Venusberg (171 m) und Kreuzberg (158 m), nach Südwesten steigt das Stadtgebiet zum Kottenforst hin auf bis zu 190 m an. Der tiefste Bodenpunkt befindet sich mit 45,6 m ü. NHN auf der Landzunge Kemper Werth an der Siegmündung.

Nachbargemeinden

Zehn Städte und Gemeinden grenzen an die Gemarkung Bonns, die alle - bis auf Remagen, das im Landkreis Ahrweiler im Land Rheinland-Pfalz liegt - zum nordrhein-westfälischen Rhein-Sieg-Kreis gehören:

Bornheim
Niederkassel
Troisdorf
Sankt Augustin
Alfter
Königswinter
Meckenheim
Wachtberg
Bad Honnef,
Remagen (RLP)

Stadtgliederung und -zuordnung

Bonn ist eine kreisfreie Stadt mit dem Kraftfahrzeugkennzeichen BN.

Bonn ist nach § 3 der Hauptsatzung in vier Stadtbezirke unterteilt, die aus insgesamt 51 Ortsteilen bestehen. Jeder Stadtbezirk verfügt über eine eigene Bezirksvertretung mit einem Bezirksbürgermeister. Daneben besteht die Stadt aus 65 statistischen Bezirken, die teilweise den Ortsteilen im Namen und der Größe ähnlich sind. Zusätzlich wird Bonn von der städtischen Statistikstelle in neun Stadtteile aufgeteilt: Bonner Zentrumsbereich, Bonn-Südwest, Bonn-Nordwest, Bundesviertel, Godesberger Zentrumsbereich, Godesberger Außenring, Beueler Zentrumsbereich, Beueler Außenring und Hardtberg.

Auf dem Gebiet der Stadt Bonn bestehen 21 Gemarkungen in den Grenzen ehemaliger Gemeinden.

Bonn gehört dem Regierungsbezirk Köln an. Die Bezirksregierung mit Sitz in Köln übt als Landesmittelbehörde die Kommunalaufsicht u. a. über den Haushalt der Stadt Bonn aus. Ferner übt die Bezirksregierung die Schulaufsicht über die Schulen Bonns aus.

Weiterhin gehört Bonn dem Landschaftsverband Rheinland (LVR) mit Sitz ebenfalls in Köln an. Der LVR nimmt als Teil der kommunalen Selbstverwaltung für Bonn u. a. Aufgaben im Bereich sozialer Einrichtungen, zum Beispiel die Trägerschaft von Fach- und insbesondere psychiatrischen Krankenhäusern oder Förderschulen für behinderte Kinder, wahr. Weiterhin werden zum Beispiel die Aufgaben der Denkmalpflege für Bonn durch den LVR wahrgenommen.

Siedlungsgeographie und Raumplanung

Bonn bildet den südlichen Rand der Metropolregion Rhein-Ruhr, die als ein polyzentrischer Verdichtungsraum in Nordrhein-Westfalen verstanden wird und sich entlang den namensgebenden Flüssen Rhein und Ruhr erstreckt. Die Metropolregion Rhein-Ruhr umfasst ein Gebiet von etwa 7.000 km² mit mehr als zehn Millionen Einwohnern, zählt zu den fünf größten Metropolregionen Europas und ist unter den elf Metropolregionen in Deutschland die bevölkerungsreichste. Sie liegt zudem mitten im zentralen europäischen Wirtschaftsraum, der sogenannten Blauen Banane. Zum Verdichtungsraum Bonn zählen Teile der rechtsrheinischen Städte Sankt Augustin und Königswinter.

Klima

Großräumig betrachtet gehört Bonn zum atlantisch-maritimen Klimabereich, d. h. das Klima ist mild sowie allgemein warm und gemäßigt. "Cfb" lautet die Köppen-Geiger-Klassifikation.

Dies bedeutet schneearme Winter mit durchschnittlich 56 Frosttagen (niedrigste Temperatur unter 0 Grad Celsius) und nur zehn Eistagen (Tageshöchsttemperatur unter 0 Grad) bei einer durchschnittlichen Januartemperatur von 2,0 Grad. Die mittlere Temperatur im Juli liegt bei 17,6 Grad Celsius, die durchschnittliche Jahrestemperatur bei 10,0 Grad. Somit zählt Bonn zu den wärmsten Regionen Deutschlands. Entsprechend früh setzen die Blützeiten im Frühling ein.

Bezüglich der Niederschläge liegt Bonn im Regenschatten der südlich angrenzenden Mittelgebirgslandschaft. Während die Stadt einen mittleren Jahresniederschlag von nur 742 Millimetern aufweist, liegen die jährlichen Niederschläge in der Eifel bei über 800 Millimetern.

Belastend auf die Menschen wirkt sich die stets hohe relative Luftfeuchtigkeit aus. Mit durchschnittlich 35 schwülen Tagen liegt Bonn weit vor anderen deutschen Städten. Im Volksmund wird hierbei vom "Bonner Reizklima" gesprochen. Bonner wissen, dass dieser Effekt in der tiefsten Stadtlage, in einem ehemaligen Rheinarm, im Gebiet um den Hauptbahnhof, am stärksten wahrgenommen werden kann.

Für die übermäßige Schwüle ist unter anderem die unzureichende Luftbewegung im Talkessel verantwortlich, da die meist aus dem Westen stammende Frischluft durch die nördlichen Ausläufer der Eifelberge abgebremst wird. Der Talkessel ergibt sich aus der Topografie: In Bonn endet das Untere Mittelrheintal, das hier in die Kölner Bucht übergeht. Die geringe Luftbewegung beeinflusst wiederum die innerstädtische Erwärmung, so dass die Temperaturen innerhalb des Stadtgebietes beispielsweise im Juli durchschnittlich 3 bis 5 Grad Celsius höher liegen als im Umland.

In den Wintermonaten und während der Schneeschmelze tritt der Rhein häufig über seine Ufer. Bei Überschwemmungen sind insbesondere Straßen und Häuser in den Stadtteilen Mehlem (linksrheinisch) und Beuel (rechtsrheinisch) gefährdet.

Das regionale Klima mit den Besonderheiten, im Winter schneearm und im Sommer schwül, sorgt auch für typisch lakonische Bonner Redensarten, wie das bekannte: "Entweder es regnet, oder die Schranken sind runter."

Name

Die erste Erwähnung des römischen Ortes Bonna stammt aus den Historien des Tacitus und bezieht sich auf das Jahr 96 n. Chr. Aus dem römischen Ortsnamen ist der heutige Name der Stadt Bonn direkt abzuleiten. Allerdings war die Entwicklung nicht kontinuierlich. So wurde die Stadt im Mittelalter phasenweise als Bern beziehungsweise latinisiert als Verona bezeichnet, was historische Stadtsiegel belegen. Die erste Erwähnung als oppidum Bonnense, also als "Stadt Bonn", stammt aus dem Jahr 1211.

Geschichte

Chronologie

m Jahr 1989 feierte Bonn seinen 2000. Geburtstag. Die Stadt erinnerte damit an die Errichtung eines ersten befestigten römischen Lagers am Rhein 12 v. Chr., nachdem bereits 38 v. Chr. der römische Statthalter Agrippa an der Stelle Ubier angesiedelt hatte. Doch lebten im Bereich des heutigen Stadtgebietes schon sehr viel früher Menschen. Davon zeugen das 14.000 Jahre alte Doppelgrab von Oberkassel sowie ein Graben und Holzpalisaden, die im Bereich des Venusberges nachgewiesen wurden und aus der Zeit um 4080 v. Chr. stammen.

War in der Zeit vor Christi Geburt die römische Präsenz in Bonna noch bescheiden, so sollte sich das nach der Niederlage der Römer in der Varusschlacht im Jahr 9 n. Chr. ändern. In den folgenden Jahrzehnten wurde hier eine Legion stationiert, die im nördlichen Bereich des heutigen Bonn das Legionslager Bonn errichtete. Um das Lager herum und südlich davon entlang der heutigen Adenauerallee siedelten Händler und Handwerker in einem vicus.

Mit dem Ende des römischen Reiches ging der Niedergang Bonns in der Spätantike und im Frühmittelalter einher. Während der Raubzüge der Wikinger in den Rheinlanden wurde Bonn 882 zweimal gebrandschatzt und 883 die soeben wieder aufgebaute Stadt ein weiteres Mal von den Normannen überfallen, gebrandschatzt und ausgeplündert.

In fränkischer Zeit und endgültig im 9. und 10. Jahrhundert entwickelte sich im Bereich des Bonner Münsters ein geistliches Zentrum, die Villa Basilika, und im Bereich des heutigen Marktes eine Marktsiedlung. 1243 gilt als das Jahr der Verleihung vollständiger Stadtrechte.

Große Bedeutung für die weitere Entwicklung der Stadt hatte der Ausgang der Schlacht bei Worringen im Jahr 1288. Die Kölner Kurfürsten machten Bonn - neben Brühl und Poppelsdorf - zu einem ihrer Wohnsitze und schließlich zu ihrer Residenz. Die von den Kurfürsten im 17. und 18. Jahrhundert erbauten prunkvollen Paläste verliehen der Stadt ihren barocken Glanz.

Mit der Besetzung durch französische Truppen am 8. Oktober 1794 endete diese Epoche. Es folgten knapp zwei Jahrzehnte der Besatzung durch die Truppen Napoleons. Die Besatzungsabgaben an Lebensmitteln, Kleidung und Unterkünften sowie der Verlust der kurfürstlichen Landesverwaltung führten zu einer Verarmung der Bevölkerung und einer Abnahme der Einwohnerzahl um rund 20 %. Die Franzosen brachten ein bürgerliches Gesetzbuch (Code civil) und kommunalpolitische Munizipalverfassung nach Bonn. Noch unter französischer Besatzung kam es zur Ansiedlung mittlerer und größerer Industriebetriebe, insbesondere aus der Textilindustrie. Die Franzosen betrieben zudem eine konsequente Säkularisation: Liegenschaften des geistlichen Kurstaates, vor allem die kurfürstlichen Bauten, gelangten in staatlichen Besitz. Rechtsrheinische Gebiete des heutigen Bonn in Vilich kamen in den Besitz des Fürsten von Nassau-Usingen: Oberkassel gehörte zum Herzogtum Berg, einem französischen Satellitenstaat. Durch den Vertrag von Lunéville vom 9. Februar 1801 wurde auch bei Bonn der Rhein zur französischen Ostgrenze. Bonn wurde Sitz einer Unterpräfektur im neugebildeten Rhein-Mosel-Departement.

Nach den Niederlagen der französischen Armee in Russland (1812) und bei der Völkerschlacht bei Leipzig räumten die Franzosen im Januar 1814 Bonn.

Im Zuge der Beschlüsse des Wiener Kongresses fiel Bonn 1815 an Preußen. Die Stadt wurde in den nächsten Jahrzehnten geprägt von der am 18. Oktober 1818 durch die preußische Regierung neugegründeten Universität. Stifter und Namensgeber war König Friedrich Wilhelm III. von Preußen. Ende des 18. Jahrhunderts hatte es in Bonn eine Universität gegeben, die mit der französischen Besatzung 1794 geschlossen wurde. Die preußische Neugründung schloss nicht an die Hochschule aus kurfürstlicher Zeit an, sondern war Teil eines Gründungsprogramms, zu dem die Friedrich-Wilhelms-Universität Berlin und die Schlesische Friedrich-Wilhelms-Universität Breslau gehörte. Der Zusatz Rheinische im Namen der Bonner Hochschule sollte sie als Schwester der Berliner und Breslauer Universitäten ausweisen. Tatsächlich wurde sie in den folgenden 100 Jahren der bevorzugte Studienort der Hohenzollern-Prinzen. Man nannte sie auch "Prinzenuniversität", weil sowohl der damalige preußische Prinz Friedrich Wilhelm, sein Sohn Prinz Wilhelm, sowie auch dessen vier Söhne dort studierten. Auch andere Söhne hochadeliger Familien bevorzugten im 19. Jahrhundert ein Studium an jener Universität. Vor der Gründung in Bonn war Köln der Rivale für eine Universitätsgründung gewesen. Den Ausschlag gab wohl, dass die "aufgeklärte Tradition" Bonns gegenüber dem "heiligen Köln" für eine konfessionell paritätische Hochschule besser geeignet erschien. Aber auch rein praktische Gründe sprachen für Bonn: Mit dem alten kurfürstlichen Schloss und dem Poppelsdorfer Schloss gab es bereits geeignete Liegenschaften.

Professoren, Studenten, Beamte und Offiziere kamen ab 1815 nach Bonn. Darunter zahlreiche Protestanten aus den preußischen Provinzen, was im "katholischen" Rheinland eine Besonderheit darstellte. Die Preußen machten Bonn auch zur Garnisonsstadt. Im Zuge dessen wurde Bonn auch als Ruhesitz von Militärs beliebt. Auch der Fremdenverkehr erhielt nach der Reichsgründung 1871 im Zuge der "Rheinromantik" jener Jahre einen Aufschwung.

Nach dem Ersten Weltkrieg wurde die Stadt zunächst von Kanadiern, dann von Briten und schließlich bis 1926 von Franzosen besetzt.

Mehr als 1.000 Bonner, zum Großteil Bürger jüdischer Abstammung wurden während der Zeit des Nationalsozialismus ermordet (Holocaust). Etwa 8000 Personen mussten ihre Heimatstadt verlassen, wurden verhaftet oder in Konzentrationslagern eingesperrt. Als am 9. März 1945 mit dem Einmarsch amerikanischer Truppen für Bonn der Zweite Weltkrieg beendet wurde, lag der Zerstörungsgrad der Gebäude bei 30 Prozent. Von diesen waren 70 Prozent leicht bis schwer beschädigt und 30 Prozent vollkommen zerstörte Wohngebäude. Mehr als 4000 Bonner waren bei Bombenangriffen gestorben. Am 28. Mai 1945 wurde Bonn Teil der britischen Besatzungszone, anschließend bestand von 1949 bis 1955 die Enklave Bonn.

Nach dem Zweiten Weltkrieg erlebte die Stadt einen rasanten Auf- und Ausbau, besonders nach der Entscheidung für Bonn als vorläufiger Regierungssitz der neuen Bundesrepublik Deutschland statt Frankfurt am Main am 29. November 1949 (siehe Hauptstadtfrage der Bundesrepublik Deutschland). - Infolge des mit dem Gesetz zur Umsetzung des Beschlusses des Deutschen Bundestages vom 20. Juni 1991 zur Vollendung der Einheit Deutschlands (Berlin/Bonn-Gesetz) verbundenen Wegzugs von Parlament, Teilen der Regierung, einem großen Teil der diplomatischen Vertretungen und vieler Lobbyisten sowie der Privatisierung der Bundespost hat die Stadt zum Jahrtausendwechsel erneut einen Wandel durchgemacht. Die verbliebenen Ministerien, hinzugezogene Bundesbehörden, Verwaltungszentralen großer deutscher Unternehmen, internationale Organisationen und Institutionen der Wissenschaft und der Wissenschaftsverwaltung sind die Träger dieses Strukturwandels, der bisher als erfolgreich gewertet wird und bis heute andauert.

Eingemeindungen

Die Stadt Bonn wurde mehrmals durch Eingemeindungen vergrößert. Um 1900 war Bonn stark gewachsen. In der Folge wurden am 1. Juni 1904 die Orte Poppelsdorf, Endenich, Kessenich und Dottendorf eingemeindet, mit denen Bonn baulich zusammengewachsen war.

Durch die mit dem Gesetz zur kommunalen Neugliederung des Raumes Bonn ("Bonn-Gesetz") einhergehende Gebietsreform vom 1. August 1969 wurde die Einwohnerzahl der Stadt etwa verdoppelt und der Siegkreis mit dem Landkreis Bonn zum Rhein-Sieg-Kreis zusammengelegt. Die einst selbstständigen Städte Bad Godesberg und Beuel und die Gemeinde Duisdorf wurden eigene Stadtbezirke von Bonn.

Der auf der rechten Rheinseite gelegene Stadtbezirk Beuel erhielt zusätzlich die Ortschaften Holzlar, Hoholz und das Amt Oberkassel zugeschlagen, die bis dahin zum Siegkreis gehörten. Bonn selbst wurde um die Orte Ippendorf, Röttgen, Ückesdorf, Lessenich/Meßdorf und Buschdorf des ehemaligen Landkreises Bonn erweitert, Lengsdorf und Duisdorf bildeten zusammen mit einigen Neubaugebieten den Stadtbezirk Hardtberg.

Die Stadt Bad Godesberg hatte zuvor ihrerseits mehrere Orte eingemeindet. Bereits 1899 waren Plittersdorf und Rüngsdorf zu Godesberg gekommen, 1904 kam noch Friesdorf hinzu, womit Bad Godesberg faktisch bereits mit Bonn zusammengewachsen war. Im Jahr 1915 war Bad Godesberg nach Südwesten aus dem Tal hinausgewachsen, so dass Muffendorf eingemeindet wurde. Am 1. Juli 1935 wurden Lannesdorf und Mehlem Stadtteile von Bad Godesberg.

Bevölkerung

Einwohnerentwicklung

Mit 336.465 (31. Dezember 2022) gehört Bonn zu den mittleren Großstädten und zu den zehn größten Städten in Nordrhein-Westfalen und ist ein Oberzentrum.

Die Einwohnerzahl der Stadt Bonn überschritt 1934 erstmals die 100.000-Grenze, womit sie von einer Mittel- zur Großstadt wurde. Durch Eingemeindungen verdoppelte sich die Einwohnerzahl bis 1969. Im Vorfeld des Regierungswegzuges kam es zwischen 1992 und 1995 zu einem leichten Bevölkerungsrückgang, der zeitnah ausgeglichen wurde.

Heute gehört Bonn zu den Großstädten in Deutschland mit nach wie vor wachsender Einwohnerzahl - laut Bevölkerungsprognose des Landesamtes für Datenverarbeitung und Statistik Nordrhein-Westfalen wird Bonn im Jahre 2025 etwa 342.232 Einwohner haben. Die Nachfolgeprognose für 2030 sagt für Bonn eine Einwohnerzahl von 351.801 voraus, womit Bonn zur siebtgrößten Stadt in Nordrhein-Westfalen würde. Da im Bereich des Stadtgebietes nur noch vergleichsweise wenig bebaubare Flächen vorhanden sind, ist nicht sicher, dass ein solcher Anstieg der Einwohnerzahlen tatsächlich realisiert werden kann, so dass die Umlandgemeinden das Wachstum aufnehmen müssten.

Am 1. Januar 2019 zählten 330.244 Einwohner zur wohnberechtigten Bevölkerung Bonns. Der Anteil an Frauen lag am Stichtag bei 51,7 Prozent, jener der Männer bei 48,3 Prozent. Das Durchschnittsalter lag bei 41,9 Jahren. Der prozentuale Ausländeranteil (melderechtlich registrierte Einwohner ohne deutsche Staatsangehörigkeit) bezifferte sich am 1. Januar 2019 auf 16,9 Prozent (55.704 Personen), während der Anteil der Bevölkerung mit Migrationshintergrund bei 29,3 Prozent (96.919 Personen) lag. Die Zuwanderer stammten aus 177 Ländern. Zu den größten Ausländergruppen zählten Personen aus der Türkei (8.319 Personen bzw. 8,6 Prozent), aus Syrien (7.846 Personen bzw. 8,1 Prozent), Polen (7.218 Personen bzw. 7,4 Prozent) und Marokko (5.742 Personen bzw. 5,9 Prozent). Zum 7. Dezember 2016 lebten 3.017 Asylbewerber und Flüchtlinge aus über 40 Nationen in Bonn; rund ein Drittel der von der Stadt untergebrachten Flüchtlinge stammt aus Syrien.

Konfessionsstatistik

Am 31. Dezember 2022 waren 29,6 % der Bonner Bevölkerung römisch-katholisch, 16,8 % evangelisch und 11,5 % islamisch. 3,4 % gehörten einer sonstigen sowie 38,7 % keiner Glaubensgemeinschaft an. Drei Jahre vorher bekannten sich 33,2 % der Bonner Bevölkerung katholisch, 18,6 % evangelisch und 10,8 % islamisch. 3,4 % gehören einer sonstigen sowie 33,9 % keiner Glaubensgemeinschaft an.

Christentum
Bonn ist das Zentrum der Altkatholischen Kirche in Deutschland - Bonn ist ihr Bischofssitz - und der Griechisch-Orthodoxen Metropolie von Deutschland - Bonn ist Sitz des Metropoliten.

Dialekt
Der Bonner Dialekt ist das ripuarische Bönnsch, ein mittelfränkischer Dialekt, der sich vom eng verwandten Kölsch neben einigen Vokabeln durch den ausgeprägteren Singsang, Weichheit von Konsonanten (im Gegensatz zum sehr harten Aachener Dialekt zum Beispiel) und die gemächlichere Sprechgeschwindigkeit unterscheidet. Belegt ist dies durch den in Köln geborenen Schriftsteller Ludwig Verbeek, der bis zu seinem Tode im Jahr 2020 in Bonn lebte und zu diesem Thema anmerkte:

"Westlicher Singsang, aber weniger auffällig als in Aachen. Wortschatz und Intonation sind zwar dem Kölnischen verwandt, aber weniger breit und hart."

Im Gegensatz zum selbstbewussten Köln der Handwerker war es in "vornehmen" Kreisen der Residenz- und Universitätsstadt Bonn verpönt, Dialekt zu sprechen, daher ist das Bönnsch im Alltagsleben nicht mehr so präsent wie das Kölsch in Köln. Der hohe Anteil Zugezogener (Imis) tat sein Übriges. Bekannt für seine Behandlung des bönnschen Dialekts ist der Kabarettist Konrad Beikircher, der nicht in Bonn, sondern in Südtirol geboren wurde und seit seiner Studienzeit in Bonn lebt.

Charakteristisch für Bönnsch sind die rheinische Schärfung, das Verschlucken von Endungen (Beispiel: "Bonner" = Bönne), das Transformieren des "g" zu "j" (Beispiele: "der liebe Gott" = de liwe Jott oder gut = joot), Verniedlichung mit der Endung -(s)che(n) (Beispiel: "Schnitte" = Schnittschen oder Hund = Hündsche oder "Straßenkehrer" = Kehrmännsche), sowie die Transformation des "ch" oder "g" zu "sch" (Beispiele: "Kirche" = Kirsche oder "Siegburg" = Sieschbursch (im Volksmund "die Stadt mit den drei s") oder "Technik" = Teschnik). Darüber hinaus gibt es zahlreiche lokale Wortschöpfungen, die Zugereiste vor Probleme stellen können: Ein Käsebrötchen ist ein Halwe Hahn, ein Fass Bier wird Pittermännsche genannt oder ein Roggenbrötchen wird zum Röggelsche. Aus "Zeit" wird Zigg, "weiter" heißt hierzulande wigger, "erzählen" wird zu verzelle oder "ziehen und drücken" wird zu trecke un deue. Ein Karnevalist ist ein Jeck und wenn jemand sich seltsam oder lustig verhält, dann auch als Attribut wat is der jeck (jeckisch). Touristen werden auf der Speisekarte über ein Himmel und Ärd stolpern, ein Pfannengericht von Blut- und Leberwurst. Ein erfreuter Bonner könnte "Nä, wat is dat schön!" äußern.

Eine Besonderheit, vorzugsweise in der älteren Bevölkerung, ist oder war die Verwendung von Wörtern französischen Ursprungs: Der Polizist kann auch ein Gendarm sein und der Tunnel wird durch Dehnung zum Tunnell. Der Bürgersteig ist ein Trottowar (von Französisch le trottoir), der Regenschirm ein Parraplü (fr le parapluie), das Plümmo ist ein Federbett (fr la plume = die Feder; le plumeau heute belegt als Federbesen, Staubwedel) und eine Taat (fr une tarte) ist ein Kuchen vom Blech, Beispiel Prummetaat (Pflaumenkuchen) und etwas aus dem Stegreif machen heißt us de Lamäng (fr la main = die Hand).

Persönlichkeiten

Mit Bonn verbundene Personen

An der Mauer des Jüdischen Friedhofs im Bonner Norden befindet sich ein Grabrelief des ersten namentlich bekannten Bonners, eines römischen Legionärs, der 35 n. Chr. aus Gallien kam. Die Inschrift lautet, aus dem Lateinischen übersetzt: "Dem Publius Clodius, Sohn des Publius, aus dem Stammbezirk Voltinia (in etwa heutige Provence), geboren in Alba (A. Helviorium, heute Alba-la-Romaine), Soldat der 1. Legion, 48 Jahre alt, mit 25 Dienstjahren [verstorben]. Er liegt hier begraben."

Unangefochten angeführt wird die Bonner Prominentenliste von dem Komponisten Ludwig van Beethoven. Sein Geburtshaus in der Bonngasse besuchen Jahr für Jahr viele tausend Touristen aus der ganzen Welt. Neben Beethoven wurden weitere Musiker in Bonn geboren oder haben dort ihre Heimat gefunden. Dazu zählen Andrea Lucchesi und Johanna Kinkel. Der Komponist Robert Schumann verbrachte seine letzten Lebensjahre in der damaligen Nervenheilanstalt (heute Schumannhaus) im heutigen Bonner Stadtteil Endenich und ist auf dem Alten Friedhof begraben.

Der italienische Dichter und Literaturnobelpreisträger Luigi Pirandello (1867-1936) studierte in Bonn. Von ihm ist die folgende Hommage überliefert:

"Bonn ist ein wunderschönes Städtchen am Ufer des Rheins, eines der schönsten, ja sogar das schönste überhaupt, das ich bis jetzt gesehen habe [...]"

Bonn als Geburtsort angeben können folgende Maler: Bernhard Gotfried Manskirsch, Peter Joseph Manskirsch und Peter Paul Manskirsch.

Wohnort war und ist Bonn für andere Künstler. Dazu zählte in den Jahren vor dem Ersten Weltkrieg August Macke. Heute leben und arbeiten in der Stadt Autoren wie Lars Brandt und Akif Pirinçci.

Seit mehr als 200 Jahren hat die Rheinische Friedrich-Wilhelms-Universität Bonn dazu beigetragen, dass eine große Zahl von Forschern, Lehrern und Studenten am Rhein ansässig geworden ist. Dazu gehören Ernst Moritz Arndt, August Wilhelm Schlegel, Clemens-August von Droste zu Hülshoff, Carl Schurz, Heinrich Hertz und - in neuerer Zeit - die Nobelpreisträger Wolfgang Paul und Reinhard Selten. Joseph Ratzinger (Papst Benedikt XVI.) war von 1959 bis 1963 Professor für Fundamentaltheologie in Bonn.

Neben berühmten Musikern und Wissenschaftlern wurden eine Reihe politischer Prominenter in den vergangenen Jahrzehnten am Rhein geboren oder wurden zu (Wahl-)Bonnern. Gebürtige Bonnerin ist Heide Simonis, Wahlbonner sind unter anderem der langjährige Arbeitsminister Norbert Blüm, der ehemalige Bundesfinanzminister Peer Steinbrück sowie der ehemalige Bundesminister für Wirtschaft und Arbeit, Wolfgang Clement.

Zu den namhaften Medienschaffenden mit Geburtsort Bonn gehören Moderator Johannes B. Kerner, Publizist Roger Willemsen, Comedian Luke Mockridge, Sängerin Natalie Horler, Schauspielerin Silke Bodenbender.

Politik

Sitz der Stadtverwaltung war jahrelang das im 18. Jahrhundert erbaute Rathaus am Markt, bis er aufgrund der 1969 vollzogenen Eingemeindungen 1978 ins Stadthaus in der Nordstadt verlegt wurde. Die jeweiligen Bonner Oberbürgermeister haben ihren offiziellen Sitz weiterhin im Rathaus am Markt.

Stadtoberhäupter
An der Spitze der Verwaltung und Gerichtsbarkeit Bonns standen im 12. Jahrhundert der Vogt und die zwölf Schöffen des Landesherrn. Seit 1331 sind zwei burgermeistere, später ein rat bezeugt. In einer Urkunde vom 24. Juli 1550 wurden zum ersten Mal die Zwölfter genannt, als "die zwoelf vann der gemeynden", die eine Kontrollfunktion innehatten. Sie vertraten nicht nur die Zünfte, sondern die ganze Gemeinde. Die Bürgermeister wurden vom Rat gewählt, der Rat von den Zünften und der Zwölfter von den Gemeinden. Im Salentinischen Vertrag von 1569 wurde verordnet, dass die Stadt von zwei Scheffelbürgermeisteren und zwei Ratsbürgermeisteren verwaltet werden soll, von denen jeweils einer als Regierender Bürgermeister die Geschäfte führte. Der Rat wurde auf 15 Schöffen vergrößert. Zusammensetzung und Kompetenzen des Rates veränderten sich später mehrmals. In der Zeit der französischen Besetzung ab 1794 wurde für den Bürgermeister die Bezeichnung Maire eingeführt. Nachdem die Franzosen aus der Stadt abgerückt waren, wurde am 25. Februar 1814 die französische Bezeichnung Maire durch den Titel Oberbürgermeister ersetzt. Anton Maria Karl Graf von Belderbusch hatte seit 1804 das Amt des Maire inne und war ab 1814 erster Oberbürgermeister der Stadt. In preußischer Zeit nach 1815 wurde Bonn Sitz eines Landkreises. An der Spitze der Stadt stand ab 1815 ein Oberbürgermeister, weiterhin gab es einen Rat.

Während der Zeit des Nationalsozialismus wurde der Oberbürgermeister von der NSDAP eingesetzt. Einige Straßen und Plätze wurden nach dem Gusto der Machthaber umbenannt: So wurde der heutige Konrad-Adenauer-Platz von 1934 bis 1945 Adolf-Hitler-Platz genannt. Der Reichskanzler Adolf Hitler wurde 1933 Ehrenbürger der Stadt (aberkannt 1945; 1983 bestätigt).

Nach dem Zweiten Weltkrieg setzte die Militärregierung der Britischen Besatzungszone einen neuen Oberbürgermeister ein und führte 1946 die Kommunalverfassung nach britischem Vorbild ein. Danach gab es einen von den Bürgern gewählten Rat der Stadt. Der wählte aus seiner Mitte den ehrenamtlichen Oberbürgermeister als Vorsitzenden und Repräsentanten der Stadt und einen hauptamtlichen Oberstadtdirektor als Leiter der Stadtverwaltung. 1996 wurde in Nordrhein-Westfalen die Doppelspitze in den Stadtverwaltungen aufgegeben. Der Oberbürgermeister wird nun direkt gewählt. Er ist als hauptamtlicher Oberbürgermeister Vorsitzender des Rates, Leiter der Stadtverwaltung und Repräsentant der Stadt. In der Funktion als Repräsentant wird der Oberbürgermeister in Bonn von vier Bürgermeistern vertreten. Die erste Direktwahl 1999 gewann Bärbel Dieckmann in der Stichwahl gegen den CDU-Kandidaten Helmut Stahl, 2004 wurde sie im ersten Wahlgang im Amt bestätigt. Dieckmann kandidierte bei der Wahl 2009 nicht wieder. Zu ihrem Nachfolger wurde Jürgen Nimptsch (SPD) gewählt, der sich mit 40,9 Prozent gegen den CDU-Kandidaten Christian Dürig durchsetzte. Die Stichwahl war zuvor durch die Landesregierung NRW abgeschafft worden. Im September 2015 gewann bei der anstehenden Neuwahl Ashok-Alexander Sridharan (CDU) vor den Gegenkandidaten Peter Ruhenstroth-Bauer (SPD) und Tom Schmidt (Grüne). Der bisherige Amtsinhaber Nimptsch trat nicht mehr zur Wiederwahl an. Sridharan wurde am 21. Oktober 2015 vereidigt und in sein Amt eingeführt. Zeitgleich mit den Kommunalwahlen am 13. September 2020 fand die Oberbürgermeisterwahl in Bonn statt, bei der keiner der Kandidaten die erforderliche absolute Mehrheit erzielte. In der Stichwahl am 27. September 2020 unterlag Amtsinhaber Sridharan gegen die grüne Herausforderin Katja Dörner. Ihre Vereidigung und Amtseinführung als Oberbürgermeisterin fand am 5. November 2020 statt.

Stadtrat

Der Rat der Stadt Bonn wurde am 13. September 2020 im Rahmen der Kommunalwahlen in Nordrhein-Westfalen von den stimmberechtigten Bürgern Bonns gewählt und umfasst 66 Mitglieder.

Gegenwärtig regiert in Bonn eine Koalition aus Grüne, SPD, Linke und Volt. Diese sogenannte "Traubenkoalition" stellte am 17. Januar 2021 ihren Koalitionsvertrag vor, der durch die Mitglieder der jeweiligen Parteien bestätigt wurde und am 30. Januar 2021 in Kraft trat. In der vergangenen Wahlperiode 2014 bis 2020 hatten sich CDU, Grüne und FDP (Schwarz-Grün-Gelb) zu einer Koalition zusammengeschlossen. In der vergangenen Ratsperiode von 2009 bis 2014 bildeten CDU und Grüne eine schwarz-grüne Koalition.

Dezernate und Ämter der Stadt Bonn
Die Stadtverwaltung gliedert sich in Dezernate, denen die städtischen Ämter nach Ressorts unterstehen:

Dezernat Oberbürgermeister
Dezernat I - Allgemeine Verwaltung und Ordnung
Dezernat II - Finanzen, Recht und Gesundheit
Dezernat III - Planung, Umwelt und Verkehr
Dezernat IV - Sport und Kultur
Dezernat V - Schule, Soziales und Jugend.
Die Dezernate I bis V werden von einem Stadtdirektor, der zugleich Vertreter des Oberbürgermeisters ist, einem Stadtkämmerer und drei weiteren Beigeordneten geleitet. Sie sind hauptamtlich tätig und werden vom Bonner Stadtrat gewählt.

Wahlen
Landtagswahlen
Fünf Abgeordnete vertreten die Bundesstadt Bonn in dem am 15. Mai 2022 gewählten Landtag von Nordrhein-Westfalen. Direkt gewählte Mitglieder sind Guido Déus im Wahlkreis 29 (Bonn I) und Christos Georg Katzidis (beide CDU) im Wahlkreis 30 (Bonn II). Über die Landesliste von Bündnis 90/Die Grünen sind Tim Achtermeyer und Julia Höller in den Landtag eingezogen. Über die Landesliste der FDP ist Joachim Stamp in den Landtag eingezogen, Stamp war vom 30. Juni 2017 bis zum 28. Juni 2022 zudem stellvertretender Ministerpräsident und Familienminister des Landes Nordrhein-Westfalen.

Bundestagswahlen
Bonn bildet den Bundestagswahlkreis Bonn (96). Bei der Bundestagswahl 2021 wurde der Wahlkreis durch Katrin Uhlig erstmals von den Grünen gewonnen. Über die jeweiligen Landeslisten ihrer Parteien wurden Jessica Rosenthal (SPD) und erneut wie 2017 Alexander Graf Lambsdorff (FDP) in den Deutschen Bundestag gewählt.

Der bisherige Wahlkreisabgeordnete Ulrich Kelber (SPD) hatte bereits am 6. Januar 2019 sein Mandat niedergelegt und wurde Bundesbeauftragter für den Datenschutz und die Informationsfreiheit.

Haushalt
Der Haushalt der Bundesstadt Bonn sieht im Ergebnisplan Erträge (Einnahmen) von 1.347.590.857,81 Euro im Jahr 2020 vor. Dem stehen Aufwendungen (Ausgaben) von 1.393.266.063,48 Euro gegenüber. Das Haushaltssaldo ist mit einem Wert von -45.675.205,67 Euro negativ.

Am 24. Juni 2021 hat der Rat der Bundesstadt Bonn den Doppelhaushalt für die Jahre 2021/2022 beschlossen. Zudem hat der Rat der Bundesstadt Bonn eine mittelfristige Finanzplanung für die Jahre 2020 bis 2025 sowie die dritte Fortschreibung des Haushaltssicherungskonzeptes bis 2024 beschlossen. Die mittelfristige Finanzplanung sieht in den Jahren 2021, 2023 und 2024 jeweils Überschüsse im einstelligen Millionenbereich vor (2021: 4,0 Millionen Euro; 2023: 5,2 Millionen Euro; 2024: 1,2 Millionen Euro). Finanzielle Defizite sind in den Jahren 2022 (-33,9 Millionen Euro) und 2025 (-9,0 Millionen Euro) laut mittelfristiger Finanzplanung vorgesehen. Die Bezirksregierung Köln als Aufsichtsbehörde hat den Doppelhaushalt 2021/2022 der Bundesstadt Bonn und die dritte Fortschreibung des Haushaltssicherungskonzeptes bis 2024 ohne Änderungen genehmigt.

Die Verschuldung der Bundesstadt Bonn beträgt im Jahr 2021 2,0 Milliarden Euro. Anfang Januar 2021 war jeder Einwohner Bonns mit einem statistischen Wert von 6050 Euro verschuldet.

Hoheitssymbole

Die Stadt Bonn führt laut Hauptsatzung ein Wappen, eine Flagge und ein Dienstsiegel. Ferner verwendet die Stadt Bonn für die Öffentlichkeitsarbeit ein Logo.

Wappen
Am 4. März 1971 beschloss der Rat der Stadt Bonn aufgrund des Gesetzes zur kommunalen Neugliederung des Raumes Bonn (Bonn-Gesetz) die Einführung eines neuen Wappens, das bis heute Bestand hat.

Blasonierung: "Geteilt von Silber und Rot, oben ein durchgehendes schwarzes Balkenkreuz, unten ein hersehender schreitender goldener Löwe."
Wappenbegründung: Das Wappen der Stadt Bonn zeigt in der oberen Hälfte das schwarze Kurkölnische Kreuz auf silbernem Grund, das alle Wappen des ehemaligen Kurfürstentums zierte und das die territoriale Herrschaft des Kölner Kurfürsten verdeutlichte.
Die untere Schildhälfte zeigt einen goldenen herschauenden schreitenden Löwen auf rotem Grund.

Ein mittelalterliches steinernes Abbild des Wappentiers wurde im Volksmund "Steinernes Wölfchen" genannt. Die Skulptur zeigt einen Löwen, der einen Eber schlägt. Der Kopf des Löwen ist nicht mehr vorhanden, so dass es unklar bleibt, ob er, wie der Löwe im heutigen Wappen, zum Betrachter schaut. Das Steinerne Wölfchen diente als Gerichtssymbol und befand sich vom Frühmittelalter bis zum Ende der kurfürstlichen Zeit auf der Südseite des Münsterplatzes. Heute befindet sich je ein Abguss der Skulptur am Ende der Vivatsgasse sowie im Vestibül des Alten Rathauses. Das Original ist im Bonner Stadtmuseum zu besichtigen.

Flagge

Die Beschreibung der Flagge lautet gemäß Hauptsatzung der Stadt Bonn: "Die Flagge ist gold(gelb)-rot. Die breite goldene (gelbe) Mittelbahn wird von zwei schmalen roten Bahnen begleitet. Die Flagge zeigt in der Mittelbahn das Wappen."

Logos

Seit 2009 verwendet die Stadt Bonn die Dachmarke "Freude. Joy. Joie. Bonn." Bei Themen mit Bezug zur Verwaltung wird der dreisprachige Schriftzug "Stadt. City. Ville. Bonn." verwendet. Hierbei finden die deutsche, englische und französische Sprache Berücksichtigung. Laut der Stadt Bonn soll mit der Dreisprachigkeit Bonn als deutsche UNO-Stadt und als internationaler Standort hervorgehoben werden.

Beziehungen der Stadt Bonn
Städtepartnerschaften
Die Stadt Bonn unterhält seit 1983 eine Städtefreundschaft mit Tel Aviv-Jaffa in Israel und seit 1988 eine Städtepartnerschaft mit Potsdam. Weitere Stadtteilpartnerschaften und Städtefreundschaften, die teilweise vor der Gebietsreform 1969 entstanden sind, bestehen in den einzelnen Stadtbezirken:

Stadtbezirk Bonn: Partnerschaften mit Oxford im Vereinigten Königreich seit 1947 und mit Budafok-Tétény (deutsch Promontor), dem XXII. Bezirk von Budapest in Ungarn seit 1991, sowie eine Städtefreundschaft mit Oppeln in Polen seit 1997 (Kontakte seit 1954)
Stadtbezirk Bad Godesberg: Städtepartnerschaften mit Saint-Cloud in Frankreich seit 1957, mit Frascati in Italien seit 1960, mit Windsor and Maidenhead im Vereinigten Königreich seit 1960 und mit Kortrijk in Belgien seit 1964, sowie seit 1969 eine Städtefreundschaft mit Yalova in der Türkei
Stadtbezirk Beuel: Seit 1969 Partnerschaft mit Mirecourt in Frankreich
Stadtbezirk Hardtberg: Partnerschaft mit Villemomble in Frankreich seit 1967
Neben Städtepartnerschaften pflegt Bonn Themen-Projektpartnerschaften. Neben Jugend- und Kulturaustausch besteht teilweise ein Erfahrungsaustausch in den Bereichen Ökologie, Stadtentwicklung und Katastrophenprävention. Projektpartnerschaften bestehen (Stand 2014) mit den Städten Buchara in Usbekistan, Cape Coast in Ghana, Chengdu in der Volksrepublik China, La Paz in Bolivien, Minsk in Belarus und Ulaanbaatar in der Mongolei.

Patenschaften
Am 26. Oktober 1955 beschloss der damalige Landkreis Bonn die Übernahme der Patenschaft über die frühere kreisfreie Stadt Stolp und den ehemaligen Landkreis Stolp. Am 1. Juli 1956 begann die Patenschaft während des Stolper Bundestreffens in der Stadthalle Bad Godesberg. Nach der Neuordnung des Bonner Raumes beschloss am 21. Mai 1970 der Rat der Stadt Bonn deren Fortführung.

Des Weiteren ist die Stadt Bonn namensgebender Pate für den ICE-2-Triebzug Nummer 208, ein Airbus A350 mit der Registrierung D-AIXD der Lufthansa, ein Containerschiff und den Einsatzgruppenversorger Bonn (A 1413) der Marine.

Regionale Kooperation
Bonn, der Rhein-Sieg-Kreis und der rheinland-pfälzische Landkreis Ahrweiler kooperieren insbesondere seit dem Bonn/Berlin-Beschluss von 1991 eng miteinander, auf politischer Ebene durch den Regionalen Arbeitskreis Entwicklung, Planung und Verkehr Bonn/Rhein-Sieg/Ahrweiler (:rak). Die etwa eine Million Einwohner umfassende Region wird häufig "Bonn/Rhein-Sieg" oder "Bonn/Rhein-Sieg/Ahrweiler" genannt. Der nördliche Teil des Landkreises Neuwied zählt geographisch zum Raum Bonn, im Speziellen die Verbandsgemeinden Unkel, Linz am Rhein und Asbach. Innerhalb der Region bestehen enge wirtschaftliche Verflechtungen, weshalb sich viele in Bonn und den umgebenden Kreisen gemeinsam tätige Verbände gebildet haben. Bereits seit 1993 kooperiert die Stadt ebenfalls mit der Region Köln/Bonn im Region Köln/Bonn e. V. In diesem interkommunalen Zusammenschluss haben sich die kreisfreien Städte Köln, Bonn und Leverkusen mit den fünf Kreisen Rhein-Sieg-Kreis, Rhein-Erft-Kreis, Rhein-Kreis Neuss, Oberbergischer Kreis und dem Rheinisch-Bergischen Kreis vereinigt, um die strukturpolitische Entwicklung der Region Köln/Bonn gemeinsam zu entwickeln. Aus alter Kooperationstradition ist der Landkreis Ahrweiler ständiger Gast in diesem Gremium.

Welthauptstadt für Nachhaltigkeit und Klimaschutz
Seit 2006 veröffentlicht der Oberbürgermeister der Stadt Bonn unterstützt durch die Verwaltung in der Regel alle drei Jahre einen Nachhaltigkeitsbericht. Der erste Bericht, welcher als Standortbestimmung zu verstehen ist, wurde im Jahr 2006 veröffentlicht. 2016 unterzeichnete die Stadt die Musterresolution des Deutschen Städtetages und des Rates der Gemeinden und Regionen Europas zur 2030-Agenda. Seit dem 30. Januar 2012 existiert im Beschaffungsamt des BMI (BeschA) die im Bundeskanzleramt angesiedelte Kompetenzstelle für nachhaltige Beschaffung (KNB). Die Einrichtung der KNB geht auf einen Beschluss des Staatssekretärsausschusses für nachhaltige Entwicklung vom 21. Oktober 2011 zurück. Die KNB ist die zentrale Anlaufstelle für alle Bundesressorts, Bundesländer, Kommunen und sonstige öffentliche Beschaffungsstellen, wenn es um nachhaltige öffentliche Beschaffung geht. Wichtigste Instrumente der KNB sind zum einen die webbasierte Informationsplattform nachhaltige-beschaffung.info und die Beratung von Beschaffenden im einzelnen Vergabeverfahren. Ebenso siedelten sich 2016 mit dem zentralen Kampagnenbüro für die weltweiten Entwicklungsziele und der Regionalen Netzstelle für Nachhaltigkeitsstrategien zwei weitere wichtige Institutionen in Bonn an.

Am 8. März 2016 erklärte der damalige Bundesminister des Auswärtigen und spätere Bundespräsident Dr. Frank-Walter Steinmeier:

(...) Diese Stadt hat sich zur Welthauptstadt für Nachhaltigkeit und Klimaschutz entwickelt. Für Menschheitsaufgaben, die heute drängender sind denn je."

Das Regionale Informationszentrum der Vereinten Nationen (UNRIC) ergänzt weiter:

Bonn ist in gut zwei Jahrzehnten zu einem Zentrum für globale Zukunftsthemen geworden, zu einem Nachhaltigkeits-Hub, dessen Herz die Vereinten Nationen in der Bundesstadt sind.

Ashok Sridharan (CDU) damaliger Oberbürgermeister der Stadt Bonn erklärte im Vorwort zum Nachhaltigkeitsbericht 2016-2018, welcher im Juni 2020 erschien:

Bonn wird mit dem Themenfeld der Nachhaltigkeit verbunden wie keine andere Stadt in Deutschland. (...) Die Stadt Bonn hat sich der Umsetzung der nachhaltigen Entwicklungsziele (Sustainable Development Goals, kurz SDGs) in besonderem Maße verpflichtet.

Im Juli 2019 rief der Rat der Stadt Bonn nach einem Bürgerantrag wie viele andere Städte den "Klimanotstand" aus.

Dass Nachhaltigkeit in Bonn einen hohen Stellenwert genießt, zeigt auch die große Anzahl an Initiativen in diesem Bereich, etwa das Zentrallager Sachspenden Bonn (ZeSaBo) oder auch die zahlreichen Reparaturcafés.

Am 10. und 11. September 2021 fand veranstaltet vom Verein Bonn im Wandel e. V. im Schauspielhaus in Bad Godesberg das "1. Bonner Klimaforum - Zukunftsbilder für ein lebenswertes und klimaneutrales Bonn 2035" statt. Das 2. Bonner Klimaforum wurde aufgrund der Corona-Pandemie auf den 10. und 11. Juni 2022 verschoben.

Architektur, Kultur und Sehenswürdigkeiten

Architektur
Historische Bauwerke
Am Marktplatz liegt das ab 1737 im Stil des Rokoko erbaute Alte Rathaus, eines der Wahrzeichen der Stadt. In direkter Nachbarschaft des Rathauses befindet sich die ehemalige Hauptresidenz der Kölner Kurfürsten, das Kurfürstliche Schloss - heute das Hauptgebäude der Bonner Universität.

Die mit Kastanien bepflanzte Poppelsdorfer Allee verbindet das Kurfürstliche Schloss mit dem Poppelsdorfer Schloss, das in der ersten Hälfte des 18. Jahrhunderts als Erholungsort der Kurfürsten erbaut wurde. Unterbrochen wird diese Achse durch die Bahnstrecke mit dem Hauptbahnhof, dessen 1883/84 errichtetes Empfangsgebäude heute unter Denkmalschutz steht. Auf dem Bahnhofsvorplatz befand sich ab den 1970er-Jahren bis 2019 das umstrittene Bonner Loch, das seitdem durch das Projekt "Urban Soul" ersetzt wurde.

Zwischen dem Kurfürstlichen Schloss und dem Rhein liegt der Alte Zoll, eine Bastion des ehemaligen Festungsrings. Seine exponierte Lage bietet einen bilderbuchähnlichen Blick auf den Rhein und das Siebengebirge genau am Übergang vom Mittelrhein in die Kölner Bucht.

Das Sterntor, das ursprünglich an der Mündung der Sternstraße auf den Friedensplatz stand, wurde wegen des Baus der Straßenbahn durch die Sternstraße um 1900 abgebaut und in stark abgewandelter Form unter Einbeziehung eines Rests der Stadtmauer einige Meter versetzt am Bottlerplatz wieder aufgebaut.

Oberhalb von Bad Godesberg steht die Ruine der vermutlich in ihrem Ursprung zuerst als Fluchtburg von den Franken erbauten Godesburg. Das Godesberger Rathaus besteht aus sechs verbundenen Gebäuden, die 1790 bis 1792 durch Kurfürst Max Franz als Logierhäuser für Kurgäste erbaut wurden. Das 1790 bis 1830 erbaute ehemalige kurfürstliche Kammertheater Haus an der Redoute ist heute Außenstelle des Kunstmuseums.

Bauwerke in Bundes-Bonn
Ausgewählte Bauwerke
Aufgrund der Vielzahl der zeitgeschichtlich bedeutenden Bauwerke wurde für die Besucher Bonns Informationsstationen am Geschichtsrundweg Weg der Demokratie anlegt. Der Weg der Demokratie ist ein Rundweg, der an mehreren historischen Gebäuden des ehemaligen Regierungsviertels vorbei durch das heutige Bundesviertel, insbesondere den Ortsteil Gronau, führt. Der Pfad wurde am 21. Mai 2004 eröffnet und ist ein Projekt des Hauses der Geschichte der Bundesrepublik Deutschland und der Bundesstadt Bonn. Das Konzept wurde unter Leitung von Dietmar Preißler, dem Sammlungsdirektor der Stiftung Haus der Geschichte entwickelt.

Das Zoologische Forschungsmuseum Alexander Koenig (ZFMK) ist am Weg der Demokratie die Keimzelle der Demokratie nach 1945. Es ist zwar keine Liegenschaft des Bundes, sondern es ist ein Naturkundemuseum und eine Stiftung des öffentlichen Rechts des Landes Nordrhein-Westfalen. Das Museumsgebäude befindet sich direkt an der Bundesstraße 9 am Rande des Bundesviertels, steht als Baudenkmal unter Denkmalschutz und ist eine Station des Geschichtsrundwegs Weg der Demokratie, weil am 1. September 1948 in der großen Halle des Museums der Festakt zum Zusammentritt des Parlamentarischen Rates stattfand.

Ein Kernbauwerk der alten Bundeshauptstadt ist das Parlamentsgebäude. Das Bundeshaus war ursprünglich eine pädagogische Akademie, die ab 1948 vom Parlamentarischen Rat und später von Bundestag und Bundesrat genutzt wurde. Ende der 1980er-Jahre wurde der Plenarsaal durch einen Neubau ersetzt. Seit dem Parlamentsumzug wird es als Konferenzzentrum genutzt und heißt seit 2007 World Conference Center Bonn (WCCB). Ein weiterer Teil des WCCB ist das historische Wasserwerk, dessen Pumpenhaus während des Umbaus des Bundeshauses von 1986 bis 1992 als Plenarsaal des Bundestags genutzt wurde.

Der Dienstsitz des Bundespräsidenten ist die ab 1861/1862 erbaute spätklassizistische Villa Hammerschmidt mit großem Landschaftsgarten. Die Villa Hammerschmidt in Bonn dient seit 1950 als Amts- und Wohnsitz des Bundespräsidenten der Bundesrepublik Deutschland, bis 1994 als erster und seither nach Schloss Bellevue als Zweitamts- und -wohnsitz.

Das Bundeskanzleramtsgebäude in Bonn war von 1976 bis 1999 Sitz des Bundeskanzleramtes der Bundesrepublik Deutschland und beherbergt seit 2005 das Bundesministerium für wirtschaftliche Zusammenarbeit und Entwicklung. Es liegt im Ortsteil Gronau an der Adenauerallee 139 (Bundesstraße 9) im Zentrum des Bundesviertels. Zweitsitz des Bundeskanzleramtes ist seit 2001 das zur Liegenschaft gehörende Palais Schaumburg. Es war das erste Bundeskanzleramt und diente von 1949 bis 1976 als Dienstsitz. Das Areal des ehemaligen Bundeskanzleramts, das noch einige weitere Gebäude umfasst, steht als Baudenkmal unter Denkmalschutz.

Als Kanzlerbungalow wird das ehemalige Wohn- und Empfangsgebäude des deutschen Bundeskanzlers in Bonn bezeichnet. Es wurde von 1964 bis 1999 zu diesem Zweck genutzt.

Architektonisch reizvoll ist das ehemalige, heute denkmalgeschützte Postministerium I (1954-1988). Es wurde zwischen 1953 und 1954 errichtet. Das Gebäude des Bundesministeriums für das Post- und Fernmeldewesen (offiziell Liegenschaft Adenauerallee-Nord; Adenauerallee 81-83) in Bonn war von 1954 bis 1988 Sitz des Bundesministeriums für das Post- und Fernmeldewesen sowie von 1989 bis 1999 Sitz des Auswärtigen Amtes. Seit 2000 ist es Sitz des Bundesrechnungshofs.

Am Robert-Schuman-Platz im Bundesviertel liegt das ehemalige Postministerium II (ab 1988). Das Gebäude für das damalige Bundesministerium für das Post- und Fernmeldewesen wurde nach den Plänen der Architekten Heinle, Wischer und Partner errichtet. Es dient heute als erster Dienstsitz des Bundesministeriums für Umwelt, Naturschutz und Reaktorsicherheit (BMU). Dieses Gebäude lehnt sich stilistisch an die Form eine Posthorns an, was in Luftaufnahmen erkennbar ist. Unter dem Gebäude befindet sich ein, mittlerweile außer Betrieb genommener, Atombunker.

Die Hauptverwaltung der Deutschen Post befindet sich im Post Tower, dem höchsten Bürogebäude in Nordrhein-Westfalen. Das Gebäude steht in direkter Nachbarschaft zum ehemaligen Abgeordnetenhochhaus und Wahrzeichen der Bundes(haupt)stadt, dem Langen Eugen, der seit 2002 durch die Vereinten Nationen genutzt wird.

Zwischen den beiden Hochhäusern befindet sich der Schürmann-Bau, die heutige Zentrale der Deutschen Welle. Dieses ursprünglich als Abgeordnetenbüro geplante Gebäude wurde während der Bauphase durch das Rheinhochwasser 1993 schwer beschädigt. An der Grenze der Stadtbezirke Bonn und Bad Godesberg befindet sich die Kreuzung A562/B9, die zu besonderen Anlässen mit den 191 Fahnen der UN-Staaten beflaggt ist.

Rezeption der Bauwerke
Die Bauwerke des Regierungsviertels weisen enorme Altersunterschiede und damit auch der Baustile auf. Während die Villa Hammerschmidt, Palais Schaumburg oder das Museum Koenig aus dem 19. Jahrhundert stammen, das Bundeshaus aus dem frühen 20. Jahrhundert, so kamen zunächst in der jungen Bundesrepublik wenige Gebäude (Baustopp 1955 per Gesetz), ab den 1960er viele neue Gebäude hinzu. Bis zur Wiedervereinigung 1990 erlebte Bundes-Bonn in der Spätphase einen gewissen Bauboom, um den Aufgaben der Verwaltung des Bundes gerecht zu werden und um "Gesicht zu zeigen". Es gehört wohl zu den Wechselbädern der Geschichte, dass ausgerechnet zum Zeitpunkt des stärksten Ausbaus von "Bundes-Bonn" die Wiedervereinigung über den Regierungssitz einbrach und die alte Reichshauptstadt Berlin den späteren Anspruch als Bundeshauptstadt anmeldete.

Über die Architektur der Bonner Republik gab und gibt es zahlreiche Kommentierungen, wahrscheinlich, weil es sich um einen architektonischen heterogenen "Flickenteppich" handelt, der Bundes-Bonn den Beinamen "Provisorium" einbrachte.

Es gab reichlich Kritik von allen Seiten: Peter M. Bodes urteilte: "Wohl keine Regierung in der ganzen Welt hat so viel architektonisches Chaos produziert wie der Bund in Bonn." Die Architekturkritikerin Ingeborg Flagge meinte, dass "mit der Bonner Staatsarchitektur kein Staat zu machen" sei. Und der Journalist Johannes Gross kommentierte: "In 40 Jahren wachsenden Wohlstandes hat der Staat Bundesrepublik nicht ein einziges Gebäude von architektonischem Rang errichtet." Der Architekturhistoriker Wolfgang Pehnt aber äußerte so etwas wie Verständnis: Vielleicht sei der lange Weg zur Bundeshauptstadt auch ein Abbild der Gesellschaft, "widersprüchlich in ihren Interessen, bald kleinmütig, bald zu großen Zielen aufgelegt, die sich dann wieder nicht realisieren lassen".

In ihrer Dissertation von 2015 (Buchtitel: "Bauten des Bundes 1949-1989") befasste sich Elisabeth Plessen mit der Architektur des Regierungssitzes und dokumentierte 154 realisierten und 14 geplanten Bundesbauten. Trotz des Negativimage wagte sie die Analyse, dass der Regierungssitz Bonn Ausdruck der "Stufen der Identitätsbildung einer Gesellschaft durch Architektur" sei.

Revitalisierung von alten Industrieflächen
2003 begannen auf dem Gelände der ehemaligen Oberkasseler Zementfabrik die ersten Bauarbeiten für das Städtebauprojekt Bonner Bogen. Bis Ende 2009 entstanden dort unter der Leitung des Bonner Architekten Karl-Heinz Schommer Wohn- und Bürogebäude, Veranstaltungsräume sowie das Hotel Kameha Grand Bonn. Denkmalgeschützte Gebäude der alten Fabrik blieben erhalten und wurden umfassend saniert, darunter die Direktorenvilla, das Verwaltungsgebäude und der Wasserturm.

Eine ungenutzte alte Industrieanlage ist die Auermühle in Graurheindorf.

Hohe Bauwerke
Die drei höchsten Bauwerke der Stadt sind der weithin sichtbare Funkmast des Westdeutschen Rundfunks Köln (WDR) auf dem Venusberg (180 m), der Post Tower (162,5 m) und das ehemalige Abgeordnetenhochhaus Langer Eugen (114,7 m). Der Vierungsturm des Bonner Münsters liegt mit 81,4 m auf Platz sieben der höchsten Gebäude.

Sakralbauten
Das Stadtgebiet beherbergt sehr viele Kirchen und Gotteshäuser. Nachstehend eine Auswahl:

Bonner Münster, Stiftskirche (Bonn), Schlosskirche, Namen-Jesu-Kirche (Bonn), St. Remigius (Bonn), St. Cäcilia (Oberkassel), Helenenkapelle (Bonn), Kreuzkirche (Bonn), St. Maria und Clemens (Schwarzrheindorf), St. Marien (Bonn), St. Peter (Vilich), St. Petri in Ketten (Lengsdorf), Rüngsdorfer Kirchturm, Marienkapelle (Rüngsdorf), St. Laurentius (Lessenich), Michaelskapelle (Bad Godesberg), Elisabeth-Kirche (Bonn-Südstadt), Trinitatiskirche (Endenich), St. Servatius (Friesdorf), St. Evergislus (Plittersdorf), St. Severin (Mehlem), Kreuzbergkirche (Endenich), Synagoge Bonn, Kathedrale Agia Trias (griechisch-orthodox), Al-Muhajirin-Moschee (Tannenbusch), Al-Muhsinin-Moschee (Beuel), Al-Ansar-Moschee (Bad Godesberg).

Bonn verfügt über eine Reihe von historisch bedeutenden Kirchenbauten. Ein Wahrzeichen der Stadt ist das im 11. Jahrhundert erbaute Bonner Münster. Es ist die größte aller Kirchen der Stadt und verfügt über einen Kreuzgang. Zu den ältesten Kirchenbauten in Bonn gehört die romanische Doppelkirche St. Maria und Clemens in Schwarzrheindorf. Als eine Besonderheit hat sie ein zweigeschossiges Kirchenschiff.

Die Stiftskirche, eine römisch-katholische Pfarrkirche, die den Namen St. Johann Baptist und Petrus trägt und von 1879 bis 1886 erbaut wurde, liegt am Stiftsplatz an der Kölnstraße im Ortsteil Bonn-Zentrum und prägt das Bonner Stadtbild. Die Gemeinde der Pfarrkirche ist die älteste Bonner Kirchengemeinde.

In der Remigiuskirche in der Brüdergasse, der früheren "Brüderkirche", befindet sich das Becken, in dem Beethoven getauft wurde. Oberhalb von Poppelsdorf, am Platz einer vorchristlichen Kultstätte und eines christlichen Wallfahrtsorts, erbaute Christoph Wamser 1627/28 die Kreuzbergkirche. Erzbischof und Kurfürst Clemens August ließ die Kirche in der Mitte des 18. Jahrhunderts von Balthasar Neumann durch den Anbau der Heiligen Stiege erweitern.

Die Kreuzkirche wurde 1871 als evangelische Hauptkirche der Stadt gegründet und ist heute eines der größten evangelischen Gotteshäuser des Rheinlandes.

Die Bonner Synagoge im Bonner Ortsteil Gronau wurde 1958-1959 errichtet. Sie liegt an der Tempelstraße (Hausnummern 2-4) am Nordrand des Bundesviertels, unmittelbar südlich des Auswärtigen Amts. Sie ist die einzige Synagoge der Stadt Bonn und steht als Baudenkmal unter Denkmalschutz. Bis zum Novemberpogrom vom 10. November 1938 standen Synagogen in Bonn-Stadt, Beuel, Bad Godesberg, Mehlem und Poppelsdorf.

Die 1957 wieder geweihte Alt-Katholische Kirche St. Cyprian befindet sich in der Adenauerallee. Kathedralkirche des Bischofssitzes Bonn der Altkatholischen Kirche in Deutschland ist die Namen-Jesu-Kirche in Bonn, die nach einer Renovierung am 2. Juni 2012 zur weiteren Nutzung der alt-katholischen Kirche übergeben wurde. Die Namen-Jesu-Kirche in der Bonngasse wurde im Stil der Jesuiten-Gotik als nachgotischer Kirchenbau zwischen 1686 und 1717 errichtet und befindet sich im Besitz des Landes Nordrhein-Westfalen.

Friedhöfe
Im Bonner Stadtgebiet liegen 40 städtische Friedhöfe mit einer Gesamtfläche von rund 120 Hektar. Weitere Friedhöfe werden als Pfarrfriedhöfe von Kirchengemeinden unterhalten.

Bekanntester Friedhof der Stadt ist der an der Grenze zur Nordstadt liegende Alte Friedhof: Zahlreiche Prominentengräber sowie Grab- und Denkmäler bedeutender Bildhauer machen den Alten Friedhof in Bonn zu einem der berühmtesten Friedhöfe in Deutschland. Dort befindet sich zum Beispiel das Grab von Beethovens Mutter und das Denkmal für Robert und Clara Schumann. Im 19. Jahrhundert wurde die Georgskapelle auf den Friedhof transloziert. Sie gehörte seit dem 13. Jahrhundert zu den Gebäuden der Kommende Ramersdorf. Ebenfalls eine Vielzahl an architektonisch interessanten Grabmälern und Prominentengrabstätten findet sich auf dem Poppelsdorfer Friedhof und dem Burgfriedhof in Bad Godesberg.

Muslime werden heute auf dem Nordfriedhof beerdigt. Dort liegt außerdem ein chinesisches Grabfeld. Im Sommer 2018 wurde die Anlage eines weiteren Grabfeldes für Jesiden beschlossen. Zahlreiche - teils sehr groß und aufwändig gestaltete - Gräber von Sinti und Roma befinden sich auf dem städtischen Friedhof am Platanenweg in Beuel.

Der Jüdische Friedhof in Bonn-Castell wird von Juden als Grabstätte genutzt. Auf dem Friedhof Kottenforst betreibt die jüdische Gemeinde ein Grabfeld. Reste jüdischer Friedhöfe, die von den Nationalsozialisten aufgelöst wurden oder aufgegeben wurden, befinden sich auch in der Stadt. Dazu zählen der Jüdische Friedhof in Schwarzrheindorf, der Jüdische Friedhof am Augustusring, der zu Kurkölnischen Zeiten der größte der Stadt war, der Jüdische Friedhof an der Hainstrasse in Endenich und der Jüdische Friedhof am Godesberg, der Teil des Burgfriedhofs ist.

Der älteste evangelische Friedhof weit und breit befindet sich in Bonn-Holzlar mit Gräbern von Leopold Bleibtreu und Johann Hermann Windgassen, dem Gründer der Friedrich-Wilhelms-Hütte; der älteste Grabstein dort ist von 1658.

Natur und Parkanlagen

Für die Bundesgartenschau 1979 wurden die Rheinwiesen und landwirtschaftlich genutzten Flächen südlich des damaligen Parlaments- und Regierungsviertels in einen 160 Hektar großen Landschaftspark, die Rheinaue, umgestaltet. Für die Bundesgartenschau 1979 wurde auch Flächen rechtsrheinisch von Beuel-Süd bis zur Südbrücke einbezogen. Heute dienen die Parkflächen als Naherholungsgebiet und werden für Großveranstaltungen wie Freiluftkonzerte, Feste und Flohmärkte genutzt.

Zu den historischen Parkanlagen zählen der Hofgarten mit Hofgartenwiese, südlich angrenzend an das Universitätshauptgebäude, unter Einbeziehung der Parkanlagen bis zum Alten Zoll am Rhein nach Osten hin und nach Westen, die Parkachse bis zum Poppelsdorfer Schloss mit dem Botanischen Garten. Weiterhin zählt der kleine Ernst-Moritz-Arndt-Garten zu den beliebten Parkanlagen der Stadt.

An beiden Seiten des Rheins, in Bonn und Beuel, erstrecken sich von Nord nach Süd Promenaden mit Grünflächen, die die Sicht auf die Stadt, den Rhein und das Siebengebirge erlauben.

Daneben gibt es in der Stadt einige kleinere Parkanlagen, deren größte der Kurpark in Bad Godesberg ist. Er wurde ursprünglich für den Kurbetrieb angelegt und beherbergt einige seltene Pflanzenarten. Für Bonn-Oberkassel ist das aus Privatbesitz hervorgegangene Arboretum Park Härle erwähnenswert.

Die Rheinaue, das Arboretum Härle, der Alte Friedhof und die Botanischen Gärten der Universität Bonn wurden als besonders beispielhaft in die Straße der Gartenkunst zwischen Rhein und Maas aufgenommen.

Die größte Freifläche innerhalb Bonns ist das Meßdorfer Feld zwischen Endenich, Dransdorf, Lessenich und Duisdorf. Es hat als Freifläche in Windrichtung Bedeutung für das Klima der Bonner Innenstadt und ist die einzige landwirtschaftlich genutzte Fläche im Stadtgebiet.

Weitere Erholungsgebiete sind

der westlich und südlich von Bonn gelegene Kottenforst, der 40 Quadratkilometer große Ostteil des Naturparks Rheinland, der mit einigen Ausläufern ins Bonner Stadtgebiet reicht, darunter:
der Venusberg und die Waldau und
die diese umgebenden Täler Melbtal und Katzenlochbachtal (Naturschutzgebiet)
das Rheinvorland nördlich und südlich von Beuel
sowie das südöstlich von Bonn gelegene, ebenfalls in einen Naturpark gefasste Siebengebirge mit seinen nördlichen Ausläufern.
Eine natürliche Besonderheit in der Stadt ist die Düne Tannenbusch, bei der es sich um eine 11.000 Jahre alte Binnendüne handelt. Sie entstand durch heftige Winde, die am Ende der letzten Eiszeit den Rheinsand an diese Stelle verwehten. Zum Naturschutzgebiet erklärt wurde die Düne Ende der 1980er-Jahre.

In beiden Naturparks laden weitläufige Wanderwege mit attraktiven Aussichten auf die Stadt zu Wanderungen ein. Der Fernwanderweg Rheinsteig beginnt in Bonn und durchquert im weiteren Verlauf das Siebengebirge.

Im Norden des rechtsrheinischen Bezirks Beuel grenzt Bonn an die Mündung der Sieg in den Rhein und das umgebende Naturschutzgebiet Siegaue, das als eine der letzten einigermaßen naturbelassenen Rheinmündungen Schutzstatus nach der Fauna-Flora-Habitat-Richtlinie genießt. Hier finden sich Auenwälder und Altwasser ohne besondere landwirtschaftliche Nutzung, andererseits mit hohem Artenreichtum an Flora und Fauna.

In Bonn gibt es insgesamt 47 Bäche, die meisten davon münden in den Rhein.

Kunst, Museen, Ausstellungen und Gedenkstätten

Museen und Ausstellungen

Bonn verfügt über eine große Zahl bedeutender Museen. Die Kunst- und Ausstellungshalle der Bundesrepublik Deutschland (Bundeskunsthalle) (erbaut 1986 bis 1992 vom Wiener Architekten Gustav Peichl) und das Haus der Geschichte der Bundesrepublik Deutschland gehören seit ihrer Eröffnung zu den zehn meistbesuchten Museen Deutschlands. Jährlich kommen mehr als 500.000 Besucher, bei einzelnen Wechselausstellungen übertrifft die Bundeskunsthalle diese Zahl sogar deutlich. Beide Museen entstanden Anfang der 1990er-Jahre gemeinsam mit dem städtischen Kunstmuseum Bonn und bilden zusammen mit der 1995 eröffneten und sich auf deutsche Forschung und Technik seit 1945 konzentrierenden Bonner Zweigstelle des Deutschen Museums im Wissenschaftszentrum, der ifa-Galerie und dem traditionsreichen Museum Koenig die Museumsmeile.

Auch das bundespolitische Bonn kann besichtigt werden: Der 1964 entstandene Kanzlerbungalow von Sep Ruf, zwischen der Villa Hammerschmidt und dem Palais Schaumburg unweit des Hauses der Geschichte gelegen, ist nach umfangreicher Renovierung seit 2009 der Öffentlichkeit in Führungen zugänglich. In der Innenstadt haben sich zudem einige Museen zum Verbund der CityMuseen zusammengeschlossen: Das StadtMuseum Bonn (eröffnet 1998) in der Franziskanerstraße 9, die ebenfalls dort untergebrachte Gedenkstätte für Widerstand und Verfolgung, das gegenüberliegende Ägyptische Museum, das Akademische Kunstmuseum, das Beethovenhaus und das Rheinische Landesmuseum.

In Geburts-, Wohn- und Sterbehäusern bekannter Persönlichkeiten wurden Museen eingerichtet. Das gilt für das Beethoven-Haus, für das August-Macke-Haus, das Ernst-Moritz-Arndt-Haus, das als Teil des StadtMuseum Bonn neben einem Arndt-Gedenkraum vor allem Sonderausstellungen und Veranstaltungen zu kulturhistorischen Themen des 19. Jahrhunderts bietet, und das Schumannhaus in Endenich, wo seit Jahrzehnten die Musikbibliothek der Stadtbibliothek untergebracht ist. In den Boden der Bonngasse, in der sich das Beethoven-Haus befindet, sind seit 2005 die Porträts von Persönlichkeiten eingelassen, deren Lebensläufe eng mit der Stadt verbunden sind. Im Beethoven-Haus befindet sich als Weltdokumentenerbe ein Teil des Autographen der Symphonie Nr. 9, d-Moll, op. 125 von Ludwig van Beethoven.

Die Universität verfügt über zahlreiche Museen und Sammlungen. Bekannt sind vor allem das Ägyptische Museum, eine Sammlung mit circa 3000 Originalobjekten, das Akademische Kunstmuseum, das die archäologische Sammlung der Universität beherbergt, und das Arithmeum, eine umfangreiche Sammlung von Rechenmaschinen. Der Botanische Garten gehört zur Universität. Hier ist unter anderem die größte Blume der Welt, die Titanenwurz zu bestaunen, deren Blüte 2003 als die größte Blume der Welt ins Guinness-Buch der Rekorde eingetragen wurde. Sie blüht regelmäßig, seit 2008 jedes Jahr. Weiterhin zu nennen sind das Goldfuß-Museum, eine Schausammlung von Fossilien, das Mineralogische Museum, eine Edelstein- und Meteoritensammlung, und schließlich das Horst-Stoeckel-Museum, das die Geschichte der Anästhesiologie von der Entdeckung der Äthernarkose im Jahre 1846 bis zur Gegenwart darstellt.

Mittlerweile über 40 Jahre alt ist das 1981 gegründete Frauenmuseum. Weltweit war es die erste Institution gleichen Namens oder vergleichbarer Zielsetzung. Heute kann das Frauenmuseum auf über 400 Ausstellungen zurückschauen und ist mit seinen umfangreichen Begleitprogrammen zu einer international anerkannten Institution geworden.

Das zwischen 1995 und 2003 komplett umgebaute Rheinische Landesmuseum zeigt bedeutende archäologische Denkmäler zur Kulturgeschichte des Rheinlandes und besitzt eine weniger bedeutende Sammlung zeitgenössischer Kunst aus der Region.

In der an der Poppelsdorfer Allee gelegenen Volkssternwarte Bonn werden regelmäßig öffentliche Beobachtungen des Sternhimmels und der Sonne durchgeführt.

Auf Initiative und unter Leitung der Bertolt-Brecht-Gesamtschule wurde mit Hilfe des Deutschen Zentrums für Luft- und Raumfahrt und weiteren Sponsoren in zweijähriger Arbeit im September 2002 entlang des Rheins der Bonner Planetenlehrpfad im Maßstab von 1:1 Milliarde eröffnet. Die Sonne (Durchmesser 1,40 Meter) ist Startpunkt des 5946 Meter langen Lehrpfades und steht unterhalb des Wasserwerks. In relativ kurzen Abständen zwischen 50 und 100 Metern stehen Merkur, Venus, Erde und Mars. Jupiter, Saturn, Uranus und Neptun folgen mit Abständen zwischen 700 Metern und 1,5 Kilometern. Pluto schließt den Weg am nördlichen Ende des Bonner Hafens in Graurheindorf ab. An jedem Planetenstandort sind auf Informationstafeln der Name, eine maßstabsgetreue Halbkugel, das Symbol, Durchmesser sowie alle Informationen in Brailleschrift hinterlegt.

NS-Gedenkstätten
In der Franziskanerstraße 9 befindet sich die Gedenkstätte für die Bonner Opfer des Nationalsozialismus - An der Synagoge e. V. Die informative Dauerausstellung wurde 2005 grundlegend überarbeitet und ergänzt. Sie dokumentiert Verfolgung, Leid und Ermordung der Bonner Opfer des Nationalsozialismus. Zur Gedenkstätte gehören eine Präsenzbibliothek, eine Mediothek mit Zeitzeugengesprächen sowie ein umfangreiches Archiv.

Kunst im öffentlichen Raum

Im gesamten Bereich der Stadt gibt es eine Fülle von Kunstwerken zeitgenössischer deutscher und internationaler Künstler. Dazu gehören Victor Vasarely mit seiner Fassadengestaltung des Juridicums, Henry Moore mit Large Two Forms vor dem ehemaligen Bundeskanzleramt, dem heutigen Bundesministerium für Entwicklung, und Eduardo Chillida mit De Musica IV vor dem Münster. Die Wolkenschale von Hans Arp wurde 1961 vor der Universitätsbibliothek aufgestellt. Wegen der mehrjährigen Sanierung des Gebäudes war Arps Werk zwischen 2004 und Mai 2009 nicht zu sehen.

Begünstigt wurde diese hohe Anzahl an Kunstobjekten durch die Bautätigkeit der öffentlichen Hand im Zusammenhang mit dem Ausbau Bonns zum Regierungssitz. Arbeiten, die als Kunst am Bau entstanden sind sowie Skulpturen vor öffentlichen Einrichtungen wie der Universität und den Museen und nicht zuletzt Spenden privater Mäzene, machen es möglich, dass ein Besucher beim Gang durch die Stadt einen Gang durch die Geschichte der bildenden Kunst der letzten 50 bis 60 Jahre unternehmen kann.

Zu Ehren Ludwig van Beethovens steht auf dem Münsterplatz ein Beethoven-Denkmal.

Denkmäler zu Ehren einzelner Personen beschreibt die Liste der Personendenkmäler in Bonn.

Theater, Musik, Film

Das Beethoven Orchester Bonn veranstaltet regelmäßig Konzerte in der Beethovenhalle und kommt in der Oper zum Einsatz. Es wurde 1897 als Philharmonisches Orchester Koblenz gegründet und 1907 von der Stadt Bonn als Städtisches Orchester Bonn übernommen.

Neben dem städtischen Theater Bonn mit der Oper Bonn und dem im Godesberger Schauspielhaus (ehemals Kammerspiele) betriebenen Schauspiel gibt es diverse kleinere Privattheater in Bonn. Dazu gehören das in der Innenstadt gelegene Contra-Kreis-Theater, das Euro Theater Central, das in Beuel gelegene Junge Theater Bonn, das Theater DIE RABEN, das Kleine Theater Bad Godesberg, das Theater Die Pathologie in der Südstadt, die Bonn University Shakespeare Company sowie seit September 2018 Malentes Theater Palast auf der Godesberger Allee.

Bonn beheimatet auch namhafte Chöre wie den Bach-Chor, den Bonner Jazzchor, den Chur Cölnischen Chor, das Immortal Bach Ensemble oder den Philharmonischen Chor sowie Vox Bona.

Kleinkunst und Kabarett werden unter anderem im Haus der Springmaus, im Pantheon-Theater (seit 2016 in der Halle Beuel), in der Endenicher Harmonie und im Theater im Ballsaal dargeboten. Die Figurentheaterkunst pflegen in verschiedenen Bonner Spielstätten die Piccolo Puppenspiele. Seit einigen Jahren etablierte sich in Bonn eine rege Poetry-Slam-Szene: Seit 2001 findet monatlich der Bonner Rosenkrieg statt und seit 2009 hat Bonn mit Sex, Drugs & Poetry einen zweiten Slam.

Von 1997 bis 2011 fanden im Sommer Konzerte mit deutschen und internationalen Künstlern auf dem Museumsplatz an der Bundeskunsthalle als Freiluftkonzerte unter einem Zeltdach statt. Als Nachfolgeplatz wird seit 2012 der Kunst!rasen in der Gronau am Rande der Rheinaue betrieben. Kleinere Auftritte finden in der Bad Godesberger Klangstation und der Endenicher Harmonie statt. Mit der Freiluftveranstaltung Rheinkultur verfügte das Kulturangebot der Stadt bis 2011 über eines der wichtigsten Festivals Deutschlands, auf dem praktisch alle modernen Stilrichtungen vertreten waren.

Das traditionsreiche Kino Metropol am Marktplatz wurde im März 2006 geschlossen, nachdem das Gebäude Ende 2005 in die Hand eines neuen Besitzers gewechselt ist. Nach einer scharf geführten Auseinandersetzung um Abriss, Umnutzung oder Weiternutzung der denkmalgeschützten Spielstätte wird das Gebäude nun als Buchhandlung genutzt. Die ebenfalls am Markt gelegenen Stern Lichtspiele werden von Cinestar betrieben. In dem 1956 am Bertha-von-Suttner-Platz erbauten Gebäude der Universum-Lichtspiele ist seit 1998 das Woki ansässig. Im Zentrum von Bad Godesberg befindet sich das Multiplex-Kino Kinopolis. In Bonn gibt es drei Programmkinos: das 1952 in Endenich eröffnete denkmalgeschützte Rex Lichtspieltheater, die 1933 in Beuel erbaute Neue Filmbühne und die im Kulturzentrum Brotfabrik Bonn gelegene Bonner Kinemathek.

Regelmäßige Veranstaltungen

Das Beethovenfest ist ein jährlich im Herbst stattfindendes fast vierwöchiges Musikfestival mit über 50 Konzerten in Bonn und der Umgebung.

2005 wurde zum ersten Mal der Beethoven Competition durchgeführt, ein Wettbewerb für junge Pianisten aus der ganzen Welt.

Das Bonner Schumannfest dient der Erinnerung an Robert Schumann und findet jährlich seit 1998 statt.

Seit dem Jahr 2000 findet monatlich die Orgel- und Kammermusikreihe am 7. um 7 in der Bonner Kreuzkirche statt. Weitere Orgelfeste sind das Bonner Orgelfest, das seit 2009 alle 2 Jahre an verschiedenen Orgeln im Bonner Stadtgebiet stattfindet sowie die Internationalen Orgelkonzerte Bonn-Beuel, die seit der Einweihung der neuen Oberlinger-Orgel im Jahr 1981 in St. Josef Bonn-Beuel dort jährlich mit international renommierten Organisten stattfinden.

Seit 2010 findet jährlich im Mai das Jazzfest Bonn an verschiedenen Spielstätten in Bonn statt.

Im Arkadenhof der Universität werden jedes Jahr im Sommer während der Internationalen Stummfilmtage restaurierte Stummfilme gezeigt.

Auf dem Münsterplatz fand zwischen 2005 und 2013 jährlich im Herbst die Wasserorgel-Veranstaltung Klangwelle Bonn statt. Seither findet die Veranstaltung unter dem Namen Klangwelle in der rheinland-pfälzischen Stadt Bad Neuenahr-Ahrweiler statt.

In der Rheinaue findet an jedem dritten Samstag im Monat von März bis Oktober der Große Rheinauen-Flohmarkt statt. Jährliche Veranstaltungen in der Rheinaue sind das Großfeuerwerk Rhein in Flammen am ersten Mai-Wochenende, eine Bierbörse am letzten Wochenende im Juli sowie das Internationale Begegnungsfest im Herbst. Das seit 1983 etablierte Freiluftmusikfestival Rheinkultur findet seit 2012 nicht mehr statt. Von 2015 bis 2018 war die Rheinaue Austragungsort des Rockaue Open Air. Seit 2015 findet dort einmal im Jahr das Festival Panama Open Air statt.

Seit 2008 findet in Neu-Vilich das Green Juice Festival statt.

Der größte jährliche Jahrmarkt in Bonn, Pützchens Markt, findet am zweiten Wochenende im September in Beuel-Pützchen auf einer Festwiese im Osten der Stadt statt. Seine Ursprünge reichen bis ins Jahr 1367. Mit rund 1,2 bis 1,4 Millionen Besuchern zählt Pützchens Markt zu den großen Jahrmärkten im Rheinland. Das Volksfest wird als "umsatzstärkster 5-Tage-Markt in Deutschland" bezeichnet.

Die AnimagiC, eine der größten deutschsprachigen Anime-Conventions (Veranstaltung für Manga- und Anime-Fans), wurde bis 2016 jährlich in der Beethovenhalle veranstaltet. Mit Beginn der dortigen Sanierungsarbeiten wanderte die Convention nach Mannheim ab. Weitere regelmäßige Veranstaltungen wie die FeenCon finden in Bonn-Beuel statt.

Die Kirschblüte in der Bonner Altstadt zieht im April und Mai Touristen aus aller Welt an. Waren es früher vorwiegend asiatische Touristen, aus deren Heimat die Bäume stammen, kommen mit wachsendem Bekanntheitsgrad auch Besucher aus anderen Ländern.

Vom vorletzten Wochenende im November (pausierend am Totensonntag) bis zum 23. Dezember findet in der Innenstadt ein Weihnachtsmarkt statt. Er erstreckt sich vom Münsterplatz über die Vivatsgasse, den Mülheimer Platz, den Bottlerplatz bis zum Friedensplatz.

Von 1970 bis 2011 fand in Bonn ein internationales vielfältiges Kulturprogramm, verteilt über mehrere Sommer-Wochenenden, unter dem Namen Bonner Sommer statt. Für Musiker, Künstler und die freie Kulturszene eine gute Gelegenheit an die Öffentlichkeit zu treten. Im Jahre 2011 stimmten im Rahmen einer Bürgerbefragung 690 Bonner gegen die Kultur-Ausgabe (ca. 300.000 Euro jährlich), 629 dafür. Für 2020 beschloss der Stadtrat im Jahr 2019 das seinerzeit beliebte Fest wiederzubeleben und mit den Stadtgartenkonzerten zu verbinden.

Brauchtum

Karneval

Bonn zählt zu den rheinischen Karnevalshochburgen, wenngleich es immer etwas im Schatten des größeren Kölner Karnevals steht.

Im Beueler Rathaus übernimmt an Weiberfastnacht die Wäscherprinzessin die Regentschaft. Das Alte Rathaus in Bonn wird seit Beginn des 20. Jahrhunderts am Karnevalssonntag von den Bonner Stadtsoldaten in historischen Uniformen im französischen Stil belagert und erobert. Die größte Karnevalssitzung ist die Alternative Karnevalssitzung Pink Punk Pantheon mit alljährlich über 10.000 Besuchern.

Einheimische definieren die Karnevalszeit zwischen dem 11. November um 11:11 Uhr und dem Aschermittwoch als "fünfte Jahreszeit".

Sportwesen

Sportvereine
Der bekannteste Sportverein Bonns ist der Basketballverein Telekom Baskets Bonn, dessen erste Herren-Mannschaft seit Jahren erfolgreich in der Basketball-Bundesliga spielt und seit 2008 die Heimspiele im 6.000 Zuschauer fassenden Telekom Dome im Ortsteil Duisdorf austrägt.

Bonn ist die größte deutsche Stadt, aus der noch nie ein Verein in der Fußball-Bundesliga spielte. Bekanntester Fußballverein ist der Bonner SC, der seine Spiele im Sportpark Nord austrägt und aktuell in der fünftklassigen Mittelrheinliga spielt. In der Saison 1976/77 spielte er einmalig in der 2. Bundesliga.

Weitere Sportvereine sind der 1. Badminton Club Beuel (Deutscher Badminton-Meister 1981, 1982 und 2005), der ehemalige Damen-Basketball-Bundesligist BG Rentrop Bonn (heute BG Bonn 92), der Baseball-Bundesligist Bonn Capitals (Deutscher Meister der Baseball-Bundesliga 2018 und 2022 und mehrfacher deutscher Meister in den Jugendklassen), der Bonner Tennis- und Hockey-Verein (Hallenhockey-Bundesligist bei den Damen), der Hockey- und Tennis Club Schwarz-Weiß Bonn, der Verein für American Football Bonn Gamecocks (ehemals zweite Bundesliga), der Rugby Club Bonn-Rhein-Sieg (2. Bundesliga West), sowie Bonns größter Sportverein, die Schwimm- und Sportfreunde Bonn 1905 (SSF Bonn), mehrfacher Deutscher Volleyball-Meister und Pokalsieger sowie Heimatverein der Olympiasiegerin im Modernen Fünfkampf 2008, Lena Schöneborn. Ebenso ist die Triathlon-Abteilung des SSF Bonn mit Damen- und Herrenteams in der Triathlon-Bundesliga vertreten.

Bester Bonner Handballverein ist die TSV Bonn rrh., die bei den Frauen und Männern 2022/23 in der Handball-Regionalliga Nordrhein spielte. In der Nähe des Sportparks Nord hat der Deutsche Fechter-Bund seine Zentrale mit angeschlossenem Internat für die Nachwuchs-Elite, die zum Teil für den Olympischen Fecht-Club Bonn an den Start geht. Hier trainierten bereits Fechtstars wie Peter Joppich und Benjamin Kleibrink. In Bonn befindet sich seit mehr als 100 Jahren der Turn- und Kraftsportverein 1906 e. V. Duisdorf. Die 1. Ringer-Mannschaft des TKSV Duisdorf trat mehrere Jahre in der ersten Bundesliga an. Der größte Tanzsportverein ist der TSC Blau-Gold Rondo im Stadtteil Beuel, der regelmäßig im Frühjahr das Traditionsturnier Goldene Rebe ausrichtet.

Die direkte Nähe des Rheins zeigt sich in mehreren Rudervereinen und vier Ruder-Arbeitsgemeinschaften (AG) der Bonner Schulen, welche sich in der AG-Bonner-Schülerrudervereine (AGBS) organisieren. Mit der Eurega hat Bonn eine weit über die Bonner Grenzen hinaus bekannte Ruderregatta, die jährlich am ersten Wochenende im Mai durch den Bonner Ruder Verein ausgerichtet wird.

Schwimmbäder
Bonn verfügt über acht Schwimmbäder: Zwei Schwimmhallen, fünf Freibäder und ein kombiniertes Hallen-/Freibad mit angegliedertem Kletterwald wie folgt:

Schwimmhallen: Beueler Bütt Beuel und Frankenbad Nordstadt
Hallen-/Freibad: Hardtbergbad Hardtberg
Freibad mit Traglufthalle: Schwimmbad Friesdorf
Freibäder: Ennertbad Pützchen, Melbbad Poppelsdorf, Panoramabad Rüngsdorf und Römerbad Castell.
Außerdem wurde den Schwimmern des SSF ein schwimmsportliches Trainingszentrum im Sportpark Nord mit einem 50-Meter-Sportbecken und einem Lehrbecken überlassen. Erste Schwimmzüge für Kleinkinder sowie Erwachsene gehören ebenso zum städtischen Kursangebot wie Stilkurse in Kraultechnik oder Aqua-Fitness-Stunden.

Sportplätze, Turn- und Sporthallen
Über das Stadtgebiet verteilt sind über 100 städtische Turn- und Sporthallen. Davon sind 81 Einfach-Turnhallen, neun Großturnhallen, neun Dreifachhallen und eine Vierfach-Halle. Des Weiteren gibt es 24 Gymnastikräume und 46 Freiluftsportplätze, darunter 13 Rasenplätze. Außerhalb der städtischen Verfügung stehen 25 privat geführte Sport- und Turnhallen.

Sportveranstaltungen
Zu den jährlichen Sportereignissen zählen die German Open im Synchronschwimmen im März, der Bonn-Marathon im April, der Bonn-Triathlon im Juni, eine Station der Beachvolleyball-Meisterschaften in Deutschland im August, sowie das Herrenflorett-Weltcupturnier "Löwe von Bonn".

Gastronomie und Nachtleben

Bonn wurde wiederholt als "Bundesstadt ohne nennenswertes Nachtleben" bezeichnet. Diese Bezeichnung ist insoweit irreführend, als die Stadt gastronomisch gut entwickelt ist und über eine Anzahl hervorragender Restaurants verfügt. Sie wurde daher 2005 vom Gault-Millau zur "Schlemmerhauptstadt Deutschlands" gewählt. Mit Rainer-Maria Halbedels Halbedel's Gasthaus in Bad Godesberg hat die Stadt ein Sterne-Restaurant aufzuweisen. Die Restaurants Yunico (im Kameha Grand Hotel), EQUU (in Gronau) und Kaspars (in Castell) werden ebenfalls mit einem Stern ausgezeichnet.

In typischen Gasthäusern wird die rheinische Küche als eine Regionalküche Deutschlands angeboten, oft unter dem Etikett "gutbürgerliche Küche". Dazu zählen Klassiker wie u. a. Rheinischer Sauerbraten (Soorbrode), Rollbraten, Flönz, Muscheln rheinische Art, Himmel un Ääd und Rievkoche (Reibekuchen).

Die "studentischen" Kneipen, Bars und Diskotheken sind in Bonn dezentral verteilt und finden sich überwiegend in der als "Altstadt" bezeichneten Nordstadt, in der Südstadt und in Poppelsdorf. Erfahrene Nachtschwärmer wechseln in den frühen Morgenstunden von diesen Standorten zu den Gastronomiebetrieben rund um den Bonner Markt, die zu früher Stunde für die Marktleute ihre Türen öffnen.

Verbindungen und Vereinigungen
Freimaurer
Seit 1775 gibt es in Bonn Freimaurerlogen. Ihnen gehörte unter anderem lokale Prominenz an, wie Karl Otto Freiherr von Gymnich, Anton von Belderbusch oder Nikolaus Simrock. Zweimal wurden die Bonner Logen zwangsweise aufgelöst, von 1814 bis 1840 durch den preußischen Kreisdirektor und Freimaurer-Gegner Rehfues und 1935 bis 1945 durch die NSDAP. Die Loge Beethoven zur ewigen Harmonie ist eine der wenigen deutschen Logen, die sich der Zwangsauflösung widersetzte und heimlich in einem Privathaus weiter arbeitete. Zurzeit gibt es in Bonn sechs Freimaurerlogen aus den verschiedenen regulären Großlogen. Es existiert außerdem eine Loge für Frauen und Männer namens Licht und Wahrheit unter dem Grand Orient de Luxembourg.

Korporationen und Verbindungen
Die Schlaraffen sind mit dem Reych Schlaraffia Castrum Bonnense vertreten.

Daneben gibt es zahlreiche Studentenverbindungen in Bonn (schlagende, nicht-schlagende oder fakultativ schlagende Korporationen) mit eigenen Häusern und unterschiedlicher weltanschaulicher Ausrichtung.

Infrastruktur
Verkehr
Luftverkehr
Der nach Konrad Adenauer benannte Flughafen Köln/Bonn liegt circa 15 Kilometer nordöstlich der Stadt und ist über die A 59, eine Schnellbuslinie und die rechtsrheinische Bahnstrecke mit Bonn verbunden. Eine weitere Anbindung an den Luftverkehr existiert durch den Flugplatz Bonn-Hangelar, der in Sankt Augustin unmittelbar an der Grenze zum Stadtbezirk Beuel liegt. Der Flugplatz wird vorwiegend von Geschäftsreisenden und Sportfliegern genutzt. Ein nicht ziviler Flugplatz besteht am Hauptsitz des Bundesministeriums der Verteidigung mit dem Heliport Bonn-Hardthöhe, der jedoch nicht mehr regulär genutzt wird. Einen zivilen Hubschrauberlandeplatz hatte von 1953 bis 1961 an der Römerstraße die belgische Fluggesellschaft Sabena mit Linienflügen über Köln nach Brüssel betrieben.

Schienen- und Busverkehr
Hauptknotenpunkte des Schienenverkehrs

Der Bonner Hauptbahnhof ist Fernverkehrshalt der Deutschen Bahn an der linken Rheinstrecke Köln-Bonn-Koblenz; der Bahnhof Siegburg/Bonn an der ICE-Strecke Köln-Rhein/Main ist von der Bonner Innenstadt mit der Stadtbahnlinie 66 in 25 Minuten über die Siegburger Bahn zu erreichen. Bei störungsbedingter Umleitung über die rechte Rheinstrecke wird ersatzweise in Bonn-Beuel gehalten. Als Nahverkehrsstrecke zweigt in Bonn die Voreifelbahn in Richtung Euskirchen von der linken Rheinstrecke ab. Im Bonner Stadtgebiet sind insgesamt 13 niveaugleiche Bahnübergänge vorhanden.

Bahnhöfe
Auf Bonner Stadtgebiet gibt es neun Bahnhöfe und Haltepunkte des Schienenverkehrs. Es bestehen im Schienenpersonennahverkehr sechs Linienverbindungen zu den umliegenden Städten im Stundentakt, die sich gegenseitig auf einen 20- bis 30-Minuten-Takt verdichten. Die Voreifelbahn verkehrt werktags im 15- bis 30-Minuten-Takt, abends und sonntags im 30- bis 60-Minuten-Takt.

Schienengüterverkehr
Vom lokalen Schienengüterverkehr ist Bonn nahezu abgeschnitten, Transitschienengüterverkehr durch das Bonner Stadtgebiet findet jedoch links- wie rechtsrheinisch in erheblichen Maße statt. Ehemals existierten über zehn Güterbahnhöfe bzw. Hafenbahnhöfe auf Bonner Stadtgebiet, betrieben von drei verschiedenen Eisenbahnen (DB, KBE (Köln-Bonner-Eisenbahn) und die "alte" RSE (eine Schmalspurbahn auch Bröltalbahn genannt)) und zusätzlich zahlreiche Anschlussgleise von Bonner Unternehmen. Übrig geblieben ist alleine der Güterbahnhof in Bonn-Beuel, der seit einigen Jahren wieder regelmäßigen und umfangreichen Frachtumschlag aufweist und in ganz Bonn und Umgebung die letzte Schnittstelle zwischen Schiene und Straße ist. Einsatzbereite Anschlussgleise für die Industrie existieren in Bonn nicht mehr, alleine ein Oldtimerersatzteil-Großhandel hat noch über ein Gleis des Bonn-Beueler Güterbahnhofs direkten Zugang zum Schienennetz und wird regelmäßig über die Schiene mit Containern beliefert.

Ausbau des Schienenverkehrs
In den nächsten Jahren und Jahrzehnten ist ein umfangreicher Ausbau des Schienennetzes in Bonn und der Region geplant. Dazu gehört der Bau der S-Bahn-Linie 13, die bisher Köln und Troisdorf über die 2004 eröffnete Flughafenschleife in dichtem Takt an den Köln/Bonner Flughafen anbindet. Mit der Verlängerung durch das rechtsrheinische Bonn bis Oberkassel sollte sie für Bonn diese Funktion übernehmen. Inzwischen verbindet die Regionalbahn 27 Bonn-Beuel und Bonn-Oberkassel umsteigefrei mit dem Köln/Bonner Flughafen (60-Minuten-Takt). Die S13 soll im 20-Minuten-Takt verkehren und ginge mit dem Neubau von zwei S-Bahnhöfen einher. Die veranschlagten Kosten des 14-Kilometer-Projekts sind von anfänglich 225 auf 434 Millionen Euro gestiegen, daher ist die Verlängerung der S13 nach Oberkassel nicht unumstritten. Nachdem die DB den avisierten Fertigstellungstermin immer wieder nach hinten korrigierte, wurde seit dem Jahr 2011 von Seiten der DB kein verbindlicher Fertigstellungstermin genannt. Im September 2014 begannen vorbereitende Arbeiten im Ortsteil Vilich-Müldorf, die die Verlegung eines Wirtschaftswegs anstelle des geplanten neuen Gleises beinhalten. Im Dezember 2014 wurde ein Finanzierungs- und Realisierungsvertrag für die S13 unterzeichnet, nunmehr ist der Baubeginn für Anfang 2017 vorgesehen. Hauptsächlicher Kritikpunkt an der Verlängerung der S13 ist neben den immensen Kosten die Tatsache, dass das Bonner Zentrum mit dem Hauptbahnhof und dem Bundesviertel mit der S13 nicht zu erreichen sein wird.

Eine Direktanbindung des Flughafens über die Südbrücke an die Bonner Innenstadt und den Hauptbahnhof ist gutachterlich in verschiedenen Versionen untersucht und mit sehr schlechten Nutzen-Kosten-Quotienten bewertet worden. Ferner kann die Südbrücke heutige S-Bahnwagen statisch nicht tragen. Eine solche Verbindung muss daher als unrealistisch angesehen werden. Eine Direktanbindung des Flughafens über die Kennedybrücke wäre nach Abschluss der Sanierung der Kennedybrücke (2011) mit Zweisystemwagen (Karlsruher Modell) technisch möglich, wird von der Bonner Ratsmehrheit aber abgelehnt (Stand: 2013) und ist daher bis jetzt (2013) nicht gutachterlich auf einen Nutzen-Kosten-Quotienten untersucht worden.

Von 2013 bis 2014 wurde die Voreifelbahn auf durchgängig zwei Gleise, verbunden mit dem Neubau von zwei Haltepunkten auf Bonner Stadtgebiet (Bonn-Endenich Nord und Bonn Helmholtzstraße), ausgebaut. Ziel ist neben der besseren Erschließung durch den Neubau der Bahnhöfe ein dichterer Takt auf der stark nachgefragten Linie. In Folge ihrer mit Abschluss des Ausbaus zunehmend innerstädtischen Erschließungsfunktion verkehrt die Voreifelbahn zwischen Euskirchen und Bonn ab dem Fahrplanwechsel im Dezember 2014 als S23, womit Bonn erstmals Anschluss an das Netz der S-Bahn Köln erhält. Seit März 2016 entstand der Haltepunkt Bonn UN Campus an der linken Rheinstrecke in Höhe der Museumsmeile im Bundesviertel, um diesen Arbeitsplatzschwerpunkt besser zu erschließen. Dieser wurde am 1. November 2017 in Betrieb genommen.

Langfristig ist vorgesehen, nach dem Bau des Kölner S-Bahn-Westrings eine linksrheinische S-Bahn zwischen Köln und Bonn einzurichten, die als S 17 die Rhein-Wupper-Bahn (RB 48) zwischen Köln und Bonn-Mehlem teilweise ersetzt. Eine vom Nahverkehr Rheinland in Auftrag gegebene Machbarkeitsstudie schlägt vor, die Linke Rheinstrecke im Bonner Abschnitt zwischen Bonn-Bad Godesberg und Bonn-Mehlem viergleisig, in den anderen Abschnitten dreigleisig auszubauen, um einen 20-Minuten-Takt realisieren zu können. Die Voreifelbahn S23 soll nach der erfolgten Elektrifizierung bis Bonn-Mehlem durchgebunden und in der Hauptverkehrszeit zu einem 10-Minuten-Takt verdichtet werden. Gleichzeitig ist geplant, den Lärmschutz deutlich auszuweiten sowie die technische Ausrüstung der Strecke zu modernisieren. Zudem sollen im Rahmen des 250-Millionen-Euro-Projekts sämtliche schienengleiche Bahnübergänge im Bonner Stadtgebiet durch Über- und Unterführungen oder Auflassungen ersetzt bzw. entfernt werden. Damit werden die derzeit sehr langen Schrankenschließzeiten von teils bis zu 20 Minuten aufgelöst und der Verkehrsfluss nachhaltig optimiert.

Straßenpersonennahverkehr

Im Straßenpersonennahverkehr besitzt Bonn heute ein Stadtbahn-/Straßenbahnnetz mit etwa sechs Linien (je nach Zählweise). In den 1950er-Jahren schrumpfte das Bonner Straßenbahnnetz durch zahlreiche Stilllegungen stark ein. Die Stammstrecke zwischen Bonn und Bad Godesberg ersetzt seit dem Frühjahr 1975 hauptsächlich die Straßenbahnlinie auf der Kaiserstraße und der B 9, sie fährt tagsüber im 10-Minuten-Takt, die abendlichen Taktzeiten wurden 2002 stark ausgedünnt. Neben innerstädtischen Verbindungen bedient die Stadtbahn Bonn Siegburg, Sankt Augustin, Königswinter und Bad Honnef mit der Linie 66. Zwei Linien verkehren auf Eisenbahnstrecken der ehemaligen Köln-Bonner Eisenbahnen nach Köln über Brühl, Hürth, Bornheim und Wesseling im 20-Minuten-Takt.

Busnetz
Bonn verfügt ebenfalls über ein sehr dichtes Stadtbusnetz mit 48 Linien (davon 6 Gemeinschaftslinien 537, 541, 550, 551, 640 und SB55), das weitestgehend im 20-Minuten-Takt bedient wird. Teilweise entstehen durch Linienbündelung Taktzeiten von fünf Minuten, zu Stoßzeiten und im Schulverkehr wird das Busnetz durch mit E gekennzeichneten "Ergänzungslinien" unterstützt. Der Spätverkehr wurde 2002 auf Beschluss der Ratsmehrheit stark ausgedünnt. Im Zuge des neuen Busnetzes wurde der Spätverkehr Ende 2008 bis zum Beginn des Nachtverkehrs wieder gestärkt. Daneben existiert ein Nachtbusnetz mit zehn Linien, die stündlich untereinander Anschlüsse herstellen. Das Nachtbus-Netz wird zum Teil durch Sponsoring finanziert, d. h. jede Sponsorlinie trägt den Namen eines Sponsors, der Bus (tagsüber im normalen Linienverkehr) trägt passende Ganzreklame. Von 1951 bis 1971 verkehrte außerdem der Oberleitungsbus Bonn in der Stadt, der einen Teil des Straßenbahnnetzes ersetzte und seinerseits von Omnibuslinien abgelöst wurde.

Verkehrsverbund (VRS)
Bonn gehört zum Tarifgebiet des Verkehrsverbunds Rhein-Sieg (VRS).

Straßennetz

Bonn ist über die Bundesautobahnen 59, 555, 562, und 565 sowie die Bundesstraßen 9, 42 und 56 an das Fernstraßennetz angebunden.

Da das Stadtgebiet vom Rhein durchtrennt wird, haben die drei Rheinbrücken der A 562 (Südbrücke, Konrad Adenauer-Brücke), A 565 (Nordbrücke, Friedrich Ebert-Brücke) und B 56 (Kennedybrücke) sowie die Rheinfähren Mehlem-Königswinter, Bad Godesberg-Niederdollendorf und Graurheindorf-Mondorf besondere Bedeutung für den innerstädtischen Verkehr. Dasselbe gilt für die Bahnunterführungen und die Viktoriabrücke, die Norden und Süden des linksrheinischen Stadtgebietes verbinden.

In Bonn sind 184.582 Kraftfahrzeuge zugelassen, darunter 156.398 Personenkraftwagen.

Das Radwegenetz der Stadt Bonn wurde zwischen 1994 und 1999 stark ausgebaut. Einige Radwege wurden jedoch inzwischen wieder zurückgebaut und teilweise durch Radfahrstreifen oder Schutzstreifen ersetzt. Bonn ist Mitglied in der Arbeitsgemeinschaft fußgänger- und fahrradfreundlicher Städte, Gemeinden und Kreise in Nordrhein-Westfalen und hat die Zielsetzung, künftig zur Fahrrad-Hauptstadt zu werden (in Anlehnung an die Radlhauptstadt in München). Dafür ist unter anderem ein stadtweites Netz von Fahrradstraßen konzipiert worden.

Wasserstraßen und Häfen

Im Norden, im Ortsteil Graurheindorf, liegt der Binnenhafen der Stadt Bonn (Hafen Bonn). Vorher war er am Alten Zoll beheimatet, in der Nähe der Kennedybrücke. Als dieser Platz für die Umschlagskapazitäten nicht mehr ausreichte, wurde er in den 1920er Jahren an einen damals noch siedlungsfreien Standort verlegt. Vorgesehen war damit die Schaffung einer größeren Industrieansiedlung sowie eines Hafenbeckens. Beides wurde nicht umgesetzt. Bis 1974 war der Hafen über eine in Buschdorf abzweigende Stichstrecke der Rheinuferbahn an das Schienennetz der KBE angebunden. Mittlerweile ist der Hafen Bonn vom Ortsteil Graurheindorf landseitig komplett umschlossen. An diesem Stromhafen werden heute überwiegend Container für den Überseetransport umgeschlagen. Die Jahresumschlagsleistung liegt über alle Güter bei circa 0,5 Mio. t.

Personenschifffahrt wird von Bonn aus von den Flotten der Köln-Düsseldorfer und der Bonner Personen Schiffahrt betrieben, zu Letzterer gehört das auffällige, einem Wal nachempfundene Schiff Moby Dick.

Belastungen durch den Verkehr
Das Zusammentreffen von mehreren großen Verkehrsadern bringt es mit sich, dass nach einer Studie des Fraunhofer-Instituts für Bauphysik aus dem Jahr 2011 Bonn die lauteste Stadt in Nordrhein-Westfalen ist und die viertlauteste in Deutschland.

Versorgungsnetze
Die Bonner Stadtwerke versorgen mit Ausnahme der Ortsteile Holzlar, Hoholz und Ungarten das Stadtgebiet mit Wasser aus der Wahnbachtalsperre. Das Gasnetz ist seit einigen Jahren im Besitz der Stadtwerke, seit 2011 wird das Stromnetz wieder vollkommen kommunal betrieben, der Stadtrat hat die Konzession des RWE für die Stadtbezirke Beuel und Bad Godesberg nicht verlängert.

Nachdem Bonn Regierungssitz geworden war, wurde das Stromversorgungsnetz zum Ring- und Maschennetz umgebaut. Die gewachsenen Strukturen dieser Netze gewährleisten eine höhere Ausfallsicherheit als vergleichbare in anderen Städten.

Trinkwasserversorgung
Die Trinkwasserversorgung wird von den Stadtwerken Bonn übernommen. Jährlich werden 22 Mio. m³ Trinkwasser abgegeben. Die Stadtwerke beziehen ihr Wasser vom Wahnbachtalsperrenverband, welcher es zu 65 % aus der Wahnbachtalsperre und aus den Grundwassergewinnungsanlagen Meindorf (27 %) und Hennefer Siegbogen (8 %) gewinnt. Das Talsperrenwasser wird in der Aufbereitungsanlage Siegelsknippen folgenden Verfahrensschritten unterzogen:

Flockung zur Vorbereitung des Ausfilterns von Mikroorganismen und Störstoffen. Verwendet werden weniger als 1 g Eisen- oder Aluminiumsalze je m³.
Filtration in Zweischichtfiltern aus Anthrazit und Quarzsand
Restentsäuerung mit Kalkwasser zur Entfernung von Kohlensäure
Desinfektion mit Chlordioxid
Das Wasser aus dem Hennefer Siegbogen wird ebenfalls in Siegelsknippen aufbereitet, in Meindorf steht eine eigene Aufbereitung zur Verfügung. Im Anschluss wird das Trinkwasser aus den drei Quellen vermischt und an das Versorgungsgebiet (insgesamt 800.000 Einwohner) abgegeben. Mit einer Gesamthärte von 4,3 bis 7,1 °dH fällt das Bonner Trinkwasser in den Härtebereich "weich".

Der Brutto-Verbrauchspreis liegt bei 1,79 Euro je Kubikmeter.

Wirtschaft
Wirtschaftsstandort

Von Mitte 1991, dem Zeitpunkt des Bonn/Berlin-Beschlusses des Bundestages, bis Mitte 2002 ist die Zahl der beschäftigten Arbeitnehmer in der Stadt Bonn um annähernd 11.400 Personen und somit 8,5 Prozent auf 145.558 angestiegen. Für 2003 gibt die Stadt noch einmal einen Zuwachs um 3118 Arbeitsplätze auf 149.016 an. Umzugsbedingte Arbeitsplatzverluste konnten ähnlich wie im benachbarten Rhein-Sieg-Kreis ausgeglichen und neue Arbeitsplätze geschaffen werden.

2013 nannte die Stadt Bonn einen Kaufkraftindex von 109,6 Prozent (Bundesdurchschnitt: 100 Prozent). Somit verfügten die Einwohner Bonns zusammen über eine allgemeine Kaufkraft in Höhe von 7,3 Milliarden Euro bzw. 22.746 Euro pro Einwohner. Der überdurchschnittliche Kaufkraftindex ist auf einen hohen Beschäftigungsgrad, einen hohen Anteil hoch qualifizierter Arbeitnehmer und einkommensstarke Arbeitsplätze zurückzuführen. Der benachbarte Rhein-Sieg-Kreis kam auf eine marginal niedrigere Kaufkraft in Höhe von 21.367 Euro pro Einwohner.

Im Jahre 2016 erbrachte Bonn, innerhalb der Stadtgrenzen, ein Bruttoinlandsprodukt (BIP) von 22,824 Milliarden Euro und belegte damit Platz 12 innerhalb der Rangliste der deutschen Städte nach Wirtschaftsleistung. Das BIP pro Kopf lag im selben Jahr bei 71.222 Euro pro Kopf (Nordrhein-Westfalen: 37.416 Euro, Deutschland 38.180 Euro) und damit weit über dem regionalen und nationalen Durchschnitt. In der Stadt waren 2016 etwa 243.200 Erwerbstätige beschäftigt. Die Arbeitslosenquote lag im Dezember 2018 bei 6,1 Prozent und damit unter dem Durchschnitt von Nordrhein-Westfalen mit 6,4 Prozent.

In den meisten Städteplatzierungen zur zukünftigen Entwicklung belegen Bonn und die Region Plätze mindestens im oberen Drittel. Dass die Region ein prosperierender Wirtschaftsstandort ist, zeigt sich daran, dass die Einwohnerentwicklung seit Jahren positiv ist.

Ermöglicht wurde die positive Entwicklung unter anderem durch die Ausgleichszahlungen des Bundes an die Region, die sich insgesamt auf etwa 1,4 Milliarden Euro belaufen. Gefördert wurden im Speziellen Wissenschaftsprojekte und Baumaßnahmen. Zudem zogen zahlreiche Bundesbehörden nach Bonn um, außerdem siedelten sich in der Bundesstadt viele internationale Institutionen und Nichtregierungsorganisationen an, unter anderem zwölf der Vereinten Nationen. Auch die Konzentration der Deutschen Post und Deutschen Telekom in Bonn trug dazu bei.

Die Dienstleistungen (ohne öffentliche Verwaltung) erreichten einen Zuwachs von 27,1 Prozent, also circa 22.400 Beschäftigten, von Juni 1991 bis Juni 2002. Mit 105.171 Beschäftigten und einem Anteil von 72,3 Prozent an allen Beschäftigten hat dieser Bereich seine dominierende Stellung in Bonn ausgebaut. Dagegen hat die öffentliche Verwaltung in diesem Zeitraum fast ein Drittel ihrer Beschäftigten verloren.

Wirtschaftsforschungsinstitute gehen davon aus, dass in Bonn in den nächsten Jahren die Zahl der Arbeitsplätze weiter steigt. Im Zukunftsatlas 2016 belegte die Stadt Bonn Platz 37 von 402 Landkreisen und kreisfreien Städten in Deutschland und zählt damit zu den Orten mit "sehr hohen Zukunftschancen". In der Ausgabe von 2019 lag sie auf Platz 28 von 401.

Tourismus
Der Tourismus in Bonn wurde während der Zeit als Regierungssitz überwiegend durch Polittourismus geprägt. Seit den 1990er-Jahren weist dieser Wirtschaftszweig hohe Wachstumsraten auf, vor allem ist die Zahl der Übernachtungen seit 1993 um 40 Prozent und sind die Ankünfte von Besuchern um 58 Prozent gestiegen. Entscheidend für den Zuwachs ist unter anderem, dass sich der Fremdenverkehr und die dort tätigen Betriebe an die neuen Gegebenheiten - im Speziellen den Regierungsumzug - angepasst haben. Der Erfolg des Bonner Tourismus wird heute neben der landschaftlich günstigen Lage an Rhein und Siebengebirge wesentlich durch den Anstieg des Passagieraufkommens am Flughafen und das Kongresswesen begründet. So entfielen von den 1,16 Millionen Hotelübernachtungen im Jahr 2005 mit 300.000 über ein Viertel auf Kongressbesucher.

Die Anzahl der Tagestouristen liegt mit neun Millionen noch wesentlich höher. Insgesamt werden durch die Touristen 176 Millionen Euro jährlich in Bonn ausgegeben. In Bonn und dem Rhein-Sieg-Kreis sind - mit steigender Tendenz - 10.475 Personen im Tourismus beschäftigt.

Arbeitsmarkt
Bonn hat seit Jahren eine der niedrigsten Arbeitslosenquoten in Nordrhein-Westfalen, im Oktober 2010 betrug sie 6,9 Prozent. Ein großer Teil der in Bonn Beschäftigten kommt als Pendler aus dem Umland, hauptsächlich aus dem Rhein-Sieg-Kreis, dem Kreis Euskirchen und dem rheinland-pfälzischen Landkreis Ahrweiler, darüber hinaus aus dem Rhein-Erft-Kreis und aus Köln. Täglich fahren 80.000 Menschen nach Bonn zur Arbeit, während 30.000 Bonner außerhalb der Stadtgrenze ihrer Beschäftigung nachgehen. Damit hat Bonn nach Köln und der Landeshauptstadt Düsseldorf den dritthöchsten Pendlerüberschuss in Nordrhein-Westfalen.

Geprägt wird der Arbeitsmarkt der Region unter anderem von den zahlreichen Bundesministerien und -behörden verbunden mit mehreren Bundesverbänden und -organisationen - der Bund ist der größte Arbeitgeber in der Region - sowie den Schwergewichten Deutsche Post AG, Deutsche Telekom und Deutsche Bank mit ihrer Niederlassung Postbank. Neben den Arbeitsplätzen im Bereich der Funktionen Bundesstadt und UN-Stadt mit den internationalen Organisationen gibt es in Bonn vergleichsweise viele im Bereich der Informations- und Kommunikationstechnologie sowie der Wissenschaft mit mehreren Forschungseinrichtungen.

Strukturwandel im Einzelhandel
Wie in zahlreichen anderen Städten der Bundesrepublik gab es ab den 1990er-Jahren im stationären Einzelhandel einen Strukturwandel. Zahlreiche ehemals inhabergeführte, alteingesessene Einzelhandelsgeschäfte verschwanden und machten Filialen von Handelsketten Platz. Auch verschwanden einige Spezialgeschäfte mit hochspezialisierten Sortimenten. Nicht zuletzt auch der zunehmende Internethandel führten zu dieser Entwicklung.

Bekannte Bonner Unternehmen

Die bedeutenden Unternehmen in Bonn sind privatisierte Staatsunternehmen: Deutsche Telekom AG, Deutsche Post AG, Postbank und Autobahn Tank & Rast GmbH.

Drittgrößter Arbeitgeber der Stadt Bonn ist die Universität Bonn (einschließlich der Universitätskliniken) und als bedeutender Arbeitgeber folgen ebenfalls die Stadtwerke Bonn.

Zum anderen sitzen in Bonn einige traditionsreiche, überregional bekannte Privatunternehmen wie die Genussmittelproduzenten Verpoorten und Kessko, die Orgelmanufaktur Klais. Der größte Süßwarenhersteller Europas, Haribo, hat seinen Gründungssitz (gegründet 1922) und einen Produktionsstandort in Bonn. Der Firmensitz befindet sich heute in der rheinland-pfälzischen Gemeinde Grafschaft.

Weitere Unternehmen von überregionaler Bedeutung sind die Weck Glaswerke (Produktionsstandort), Fairtrade, Eaton Industries (ehemals Klöckner & Moeller), die IVG Immobilien AG, Kautex Textron, SolarWorld, der Smoothie Hersteller True Fruits, Vapiano und die SER Group.

Medien

Hörfunk und Fernsehen
Der mit Abstand größte Medienbetrieb in Bonn ist die Deutsche Welle. Sie hat ihre Zentrale im Schürmann-Bau und produziert dort Hörfunksendungen, die in die ganze Welt ausgestrahlt werden, sowie ein Online-Angebot in derzeit (April 2012) 30 Sprachen. Zudem hat der Fernsehsender Phoenix seine Zentrale in der Bundesstadt, im ehemaligen Hauptstadtstudio des ZDF.

Der WDR unterhält in Bonn ein Bundesstudio und ein Regionalbüro. Am 1. Februar 2007 startete die lokale Berichterstattung in Bonn/Rhein-Sieg mit einer eigenen Lokalzeit aus Bonn.

In Bonn senden außerdem der Lokalradiosender Radio Bonn/Rhein-Sieg mit Rahmenprogramm von Radio NRW und das Hochschulradio BonnFM als Kooperationsprojekt der Rheinischen Friedrich-Wilhelms-Universität Bonn und der Hochschule Bonn-Rhein-Sieg.

Druckmedien
Mit Abstand größte Tageszeitung in Bonn ist der General-Anzeiger. Er gehört zur Rheinischen Post Mediengruppe. Lokale Berichterstattung findet der Leser außerdem in der Bonner Rundschau, im Rhein-Sieg-Anzeiger und in dem Boulevardblatt Express. Diese drei Zeitungen gehören alle zu der Kölner Mediengruppe Gruppe M. DuMont Schauberg. 2004 untersagte das Bundeskartellamt der Mediengruppe, am Bonner General-Anzeiger einen Aktienanteil zu erwerben. Nach Ansicht der Kartellbehörde hätte das Geschäft zu einer Verstärkung der marktbeherrschenden Stellung auf den Leser- und Anzeigenmärkten geführt. Am 6. Juli 2005 hob das Oberlandesgericht Düsseldorf das Veto des Bundeskartellamts auf, so dass DuMont 18 Prozent Anteile erwerben konnte. Im Gegenzug erwarb die Neusser GmbH, der Verlag des General-Anzeigers, im Rahmen einer Überkreuzbeteiligung Anteile in Höhe von 9,02 Prozent an der DuMont-Gruppe. Diese Beteiligung wurde inzwischen wieder aufgelöst. Seit dem 1. Juni 2018 ist der General-Anzeiger Teil der Rheinischen Post Mediengruppe.

Eine starke Stellung im Bereich Druckerzeugnis haben die Verlagsgruppe Rentrop (unter anderem mit dem Verlag für die Deutsche Wirtschaft) und der Stollfuß-Verlag in den Bereichen Steuer, Wirtschaft und Recht. Beide gehören zu den 100 größten deutschen Verlagen. Mit der Herausgabe von musikalischer Fachliteratur, Noten und Lehrbüchern zu Musikinstrumenten gehört der Voggenreiter Verlag zu den bekanntesten Unternehmen dieser Sparte.

Monatlich erscheinen in Bonn die Stadtmagazine Schnüss (rheinisch für "Schnauze") und Szene Köln-Bonn. Die überregionale Wochenzeitung Rheinischer Merkur stammte ebenfalls aus Bonn und wurde 2010 auf Initiative der Deutschen Bischofskonferenz als Mitgesellschafter in eine Beilage der Wochenzeitung Die Zeit umgewandelt.

Internetangebote
Online-Angebote mit lokalen Nachrichten produzieren die Bonner Tageszeitungen, der WDR und Radio Bonn/Rhein-Sieg.

Nachrichtenagenturen
Die Bundespressekonferenz hat ihre einzige Außenstelle im Tulpenfeld. Hier befindet sich eine Niederlassung der Deutschen Presse-Agentur (DPA). Außerdem arbeiten in der UN-Stadt eine Reihe von Nachrichtenagenturen im Umfeld der hier angesiedelten internationalen Organisationen, wie zum Beispiel die Katholische Nachrichten-Agentur (KNA).

Übertragungstechnik
Die Rundfunkversorgung erfolgt unter anderem über die Sendemasten auf dem Venusberg und dem Großen Ölberg. Über den Sender Bonn-Venusberg auf dem Venusberg wird die Region Bonn seit 2004 mit dem digitalen Antennenfernsehen DVB-T versorgt, das die analoge Ausstrahlung ersetzte.

Öffentliche Einrichtungen

UN- und Bundesstadt
UN-Stadt (Standort von UN-Behörden)

Der 20. Juni 1996 gilt als Geburtsstunde der Bezeichnung UN-Stadt Bonn. An diesem Tag wurde vor dem Haus Carstanjen zur Einweihung als UN-Niederlassung die UN-Flagge gehisst. Bonn selbst bezeichnet sich als "die UN-Stadt am Rhein". Für 19 Organisationen, Büros und Programme der Vereinten Nationen arbeiten hier inzwischen rund 1.000 Mitarbeiter. Bonn ist auch Sitz des UN-Klimasekretariates (UNFCCC). Die meisten Organisationen verbindet der Einsatz für eine nachhaltige Entwicklung der Erde. Sie waren zunächst hauptsächlich im Bad Godesberger Haus Carstanjen ansässig, das den wachsenden Sekretariaten auf Dauer zu wenig Platz bot. Deshalb hat die Bundesregierung 2003 entschieden, den "Langen Eugen" und das Bundeshaus als ehemalige Parlamentsgebäude den Vereinten Nationen zur dauerhaften Nutzung zu überlassen und dort einen UN-Campus zu bilden. Der Campus hat den Status eines exterritorialen Gebietes. Seit der offiziellen Eröffnung des UN-Campus im Juli 2006 sind - bis auf eine - alle anderen (18) Organisationen in den "Langen Eugen" eingezogen. Die damalige Bundeskanzlerin Angela Merkel übergab am 11. Juli 2006 offiziell an damaligen UNO-Generalsekretär Kofi Annan. Im Oktober 2013 konnte das Klimasekretariat den umgebauten Südflügel des Bundeshauses - das alte Abgeordnetenhochhaus - beziehen. Jüngste UN-Organisation in Bonn ist das am 3. März 2016 eingeweihte Wissenszentrum für nachhaltige Entwicklung der Fortbildungsakademie des Systems der Vereinten Nationen (UNSSC), welches wahrscheinlich im "kurzen Eugen" untergebracht ist. Der 17 geschossige Neubau gilt als energetischer Vorzeigebau und kostete etwa 75 Millionen Euro. Lars Heyltjes regte daher in Kölner Stadtanzeiger den Spitznamen "Teurer Eugen" an.

Die Ansiedlungen der Vereinten Nationen führten zu einem Anstieg der in Bonn tätigen internationalen Institutionen und Nichtregierungsorganisationen, von denen sich in Bonn inzwischen ungefähr 170 niedergelassen haben. Darunter befinden sich unter anderem der Deutsche Entwicklungsdienst (DED), das Deutsche Institut für Entwicklungspolitik (DIE) und die Deutsche Gesellschaft für Internationale Zusammenarbeit (GIZ), bedeutende Institute der Entwicklungshilfe, die in der Wahrnehmung ihrer Aufgaben vom Bundesministerium für wirtschaftliche Zusammenarbeit und Entwicklung (BMZ) mit Hauptsitz in Bonn unterstützt werden.

Am UN-Standort Bonn sind rund 150 Nichtregierungsorganisationen (NGOs) als Interessensvertretungen angesiedelt.

Bundesstadt (Standort von Bundesbehörden)

Seit der Verlegung des Regierungssitzes nach Berlin, geregelt durch das Berlin/Bonn-Gesetz vom 26. April 1994, haben sechs Bundesministerien weiterhin ihren ersten Dienstsitz in Bonn. Weil sich hier die Bundesrepublik Deutschland 1949 konstituierte, die Stadt für mehrere Jahrzehnte Parlaments- und Regierungssitz wurde (somit bis 1990 vorläufig die Funktion einer Bundeshauptstadt wahrnahm) und Bonn das Verwaltungszentrum, d. h. Zentrum der Ministerialverwaltung des Bundes bleiben sollte, trägt die Stadt fortan den bundesweit einmaligen Titel Bundesstadt. Zudem dürfen in den Berliner Ministerien nicht mehr Mitarbeiter beschäftigt werden als in den Bonner Ministerien, in denen etwa 10.000 Personen arbeiten. Ebenfalls durch das Gesetz geregelt wurde der Umzug von 22 Bundesbehörden aus Berlin und dem Rhein-Main-Gebiet in die Bundesstadt. Außerdem legte der Bund die Ansiedlung der Deutschen Telekom, der Deutschen Post und der Postbank per Gesetz fest.

Ihren ersten Dienstsitz in Bonn haben folgende sechs Bundesministerien: das Bundesministerium der Verteidigung (BMVg); die Bundesministerien für Ernährung und Landwirtschaft (BMEL); für wirtschaftliche Zusammenarbeit und Entwicklung (BMZ); für Umwelt, Naturschutz, Bau und Reaktorsicherheit (BMUB); für Gesundheit (BMG) und für Bildung und Forschung (BMBF). Die acht Bundesministerien mit erstem Dienstsitz in Berlin haben in Bonn einen Zweitsitz.

Viele weitere Bundesbehörden wie beispielsweise das Bundesinstitut für Arzneimittel und Medizinprodukte (BfArM), die Bundesanstalt Technisches Hilfswerk (THW), das Bundeskartellamt (BKartA), der Bundesrechnungshof (BRH), die Bundesnetzagentur (BNetzA), die Bundesanstalt für Landwirtschaft und Ernährung (BLE), das Bundesamt für Naturschutz (BfN), die Bundesanstalt für Finanzdienstleistungsaufsicht (BaFin), das Bundesamt für Sicherheit in der Informationstechnik (BSI), die Bundesanstalt für Immobilienaufgaben (BImA) und das Eisenbahn-Bundesamt (EBA) sind ebenfalls in Bonn angesiedelt.

Mit dem Bundesrat und dem Bundespräsidenten haben zudem zwei Verfassungsorgane ihren zweiten Dienstsitz in der Bundesstadt.

Zu Zeiten als Bundeshauptstadt entstanden im Süden der Stadt, zwischen Bonn und Bad Godesberg, zahlreiche Bauten für Bundesangelegenheiten und wichtige Institutionen, wie der Deutsche Bundestag und die Dienstsitze von Bundeskanzler und Bundespräsident und nicht zuletzt auch einige Botschaften lagen ab 1949 im Gebiet der Rheinaue. Im Volksmund sprachen daher die Bonner von Bonn, wenn sie die Stadt meinten und von Bundes-Bonn, wenn es um die Liegenschaften des Bundes ging. Diese räumliche Abgrenzung war allerdings schon deswegen schwierig, weil zahlreiche Ministerien und Dienststellen aus Raumnot über die ganze Stadt verteilt waren.

Wissenschaft, Bildung und Forschung

Die Rheinische Friedrich-Wilhelms-Universität Bonn wurde 1777 als Akademie gegründet und 1798 geschlossen. 1818 wurde sie neu gegründet und gehört seitdem zu den größten Universitäten Deutschlands. Zusammen mit ihrer Universitätsklinik gehört sie zu den größten Arbeitgebern in Bonn. Im Mai 2019 waren über 38.000 Studierende immatrikuliert und liegt nach der Anzahl der Studierenden auf Platz 13 (von 426) der deutschen Universitäten.

Die frühere Sternwarte der Universität beherbergt heute das Institut für Kommunikationswissenschaften sowie die Volkssternwarte Bonn.

Die Hochschule Bonn-Rhein-Sieg wurde 1995 gegründet. Obwohl sie Bonn in ihrem Namen trägt, befindet sich innerhalb der Stadt kein Studienstandort. Sitz der Hochschule ist Sankt Augustin, weitere Standorte befinden sich in Rheinbach und Hennef (Sieg).

Außerdem befinden sich in Bonn die Max-Planck-Institute für Mathematik, Radioastronomie und zur Erforschung von Gemeinschaftsgütern. Seit 2012 ist die Stadt Bonn "Korporativ Förderndes Mitglied" der Max-Planck-Gesellschaft. Des Weiteren ist Bonn seit 2009 Verwaltungssitz des Deutschen Zentrums für Neurodegenerative Erkrankungen (DZNE).

Als Ausgleichsmaßnahme für den Umzug nach Berlin wurde 1998 das Forschungszentrum caesar gegründet. Das Deutsche Institut für Entwicklungspolitik, das 1964 in Berlin gegründet worden war, zog 2000 nach Bonn um. Auf dem UN-Campus ist ein Institut der Universität der Vereinten Nationen (UNU) - das Institute for Environment and Human Security (UNU-EHS) - angesiedelt. Die Fernuniversität in Hagen, die DIPLOMA - FH Nordhessen sowie die FOM Hochschule für Oekonomie & Management unterhalten Außenstellen in Bonn.

Bis 2004 beherbergte Bonn die Fachhochschule für das öffentliche Bibliothekswesen Bonn. Diese Fachhochschule war 1921 vom Borromäusverein gegründet und 1947 vom Land Nordrhein-Westfalen staatlich anerkannt worden. Seit 1982 trug sie ihren zuletzt bekannten Namen. Im Jahre 2004 wurde die Fachhochschule jedoch aufgelöst.

Die Bibliothek für Hugenottengeschichte wurde 2008 gegründet.

Die Fortbildungsakademie des Innenministeriums des Landes Nordrhein-Westfalen ist eine landesweite Fortbildungsstelle für die Beschäftigten der Kommunen sowie der Landesverwaltung. Ihren Sitz hat sie in Herne. In Bad Godesberg befindet sich die hiervon unabhängige Bildungseinrichtung der Finanzverwaltung des Landes Nordrhein-Westfalen. Sie trägt den Namen Fortbildungsakademie der Finanzverwaltung NRW (FortAFin). Nach der Verlagerung der FortAFin zum 1. Oktober 2018 wird am bisherigen Standort in Bonn-Bad Godesberg eine Dependance der Landesfinanzschule Wuppertal entstehen.

Der Deutsche Akademische Austauschdienst (DAAD), die Deutsche Forschungsgemeinschaft (DFG), die Alexander-von-Humboldt-Stiftung, die Studienstiftung des deutschen Volkes, das Cusanuswerk, die Friedrich-Ebert-Stiftung sowie der Arbeitskreis selbständiger Kultur-Institute (AsKI) haben ihre Geschäftsstellen in Bonn.

Des Weiteren haben im politischen Bereich das Bundesministerium für Bildung und Forschung (BMBF), das Sekretariat der Kultusministerkonferenz (KMK), die Hochschulrektorenkonferenz (HRK), das Bundesinstitut für Berufsbildung (BIBB), die Bund-Länder-Kommission für Bildungsplanung und Forschungsförderung (BLK), das Deutsche Institut für Erwachsenenbildung (DIE) und die Bundeszentrale für politische Bildung (bpb) ihren Sitz in Bonn.

Brandschutz
Die Feuerwehr Bonn besteht aus der 1941 gegründeten Berufsfeuerwehr, der 1863 gegründeten Freiwilligen Feuerwehr und der Jugendfeuerwehr, die sich jeweils aus verschiedenen Einheiten mit verschiedenen Wachen zusammensetzen.

Gesundheitswesen
Die über 15 Krankenhäuser sind über die ganze Stadt verteilt. Den bedeutendsten Betrieb stellt das Universitätsklinikum Bonn dar, das über 30 Kliniken in 12 Abteilungen betreibt. Fast alle sind auf dem Venusberg untergebracht, im restlichen Stadtgebiet bestehen drei weitere Standorte. Eine weitere Großklinik ist die LVR-Klinik Bonn (bis 2009 Rheinische Kliniken Bonn, bis 1997 Rheinische Landesklinik Bonn) des Landschaftsverbandes Rheinland in Bonn-Castell. Seit 2013 besteht mit den GFO Kliniken Bonn ein weiteres Gemeinschaftskrankenhaus.

Justizbehörden

Bonn ist Sitz des Landgerichtes Bonn, dem sechs Amtsgerichte unterstehen, darunter das Amtsgericht Bonn. Daneben sind in der Stadt ein Arbeitsgericht und die Staatsanwaltschaft Bonn ansässig. Das in Bonn beheimatete Bundeszentralregister ist zum 1. Januar 2007 mit der Außenstelle des Bundesjustizministeriums im neugebildeten Bundesamt für Justiz mit Sitz in Bonn aufgegangen. Dort wird unter anderem das Bundesgesetzblatt herausgegeben. Gemäß dem Berlin/Bonn-Gesetz behält das Bundesjustizministerium weiterhin eine Außenstelle mit etwa 30 Mitarbeitern in Bonn.

Arbeitsmarktbehörden
Bonn ist außerdem Standort der Zentralen Auslands- und Fachvermittlung (ZAV) der Bundesagentur für Arbeit (BA). Im Stadtteil Duisdorf befindet sich der Hauptsitz der ZAV mit ihren bundesweit 18 Standorten.

Quelle: https://de.wikipedia.org/wiki/Bonn
```

### Bild 7

```txt
Es hatte ein Mann einen Esel, der schon lange Jahre die Säcke unverdrossen zur Mühle getragen hatte, dessen Kräfte aber nun zu Ende giengen, so daß er zur Arbeit immer untauglicher ward. Da dachte der Herr daran, ihn aus dem Futter zu schaffen, aber der Esel merkte daß kein guter Wind wehte, lief fort und machte sich auf den Weg nach Bremen: dort, meinte er, könnte er ja Stadtmusikant werden. Als er ein Weilchen fortgegangen war, fand er einen Jagdhund auf dem Wege liegen, der jappte wie einer, der sich müde gelaufen hat. "Nun, was jappst du so, Packan?" fragte der Esel. "Ach," sagte der Hund, "weil ich alt bin und jeden Tag schwächer werde, auch auf der Jagd nicht mehr fort kann, hat mich mein Herr wollen todt schlagen, da hab ich Reißaus genommen; aber womit soll ich nun mein Brot verdienen?" "Weißt du was," sprach der Esel, "ich gehe nach Bremen und werde dort Stadtmusikant, geh mit und laß dich auch bei der Musik annehmen. Ich spiele die Laute, und du schlägst die Pauken." Der Hund wars zufrieden, und sie giengen weiter. Es dauerte nicht lange, so saß da eine Katze an dem Weg und machte ein Gesicht wie drei Tage Regenwetter. "Nun, was ist dir in die Quere gekommen, alter Bartputzer?" sprach der Esel. "Wer kann da lustig sein, wenns einem an den Kragen geht," antwortete die Katze, "weil ich nun zu Jahren komme, meine Zähne stumpf werden, und ich lieber hinter dem Ofen sitze und spinne, als nach Mäusen herum jage, hat mich meine Frau ersäufen wollen; ich habe mich zwar noch fortgemacht, aber nun ist guter Rath theuer: wo soll ich hin?" "Geh mit uns nach Bremen, du verstehst dich doch auf die Nachtmusik, da kannst du ein Stadtmusikant werden." Die Katze hielt das für gut und gieng mit. Darauf kamen die drei Landesflüchtigen an einem Hof vorbei, da saß auf dem Thor der Haushahn und schrie aus Leibeskräften. "Du schreist einem durch Mark und Bein," sprach der Esel, "was hast du vor?" "Da hab ich gut Wetter prophezeit," sprach der Hahn, "weil unserer lieben Frauen Tag ist, wo sie dem Christkindlein die Hemdchen gewaschen hat und sie trocknen will; aber weil Morgen zum Sonntag Gäste kommen, so hat die Hausfrau doch kein Erbarmen, und hat der Köchin gesagt sie wollte mich Morgen in der Suppe essen, und da soll ich mir heut Abend den Kopf abschneiden lassen. Nun schrei ich aus vollem Hals, so lang ich noch kann." "Ei was, du Rothkopf," sagte der Esel, "zieh lieber mit uns fort, wir gehen nach Bremen, etwas besseres als den Tod findest du überall; du hast eine gute Stimme, und wenn wir zusammen musicieren, so muß es eine Art haben." Der Hahn ließ sich den Vorschlag gefallen, und sie giengen alle viere zusammen fort.

Sie konnten aber die Stadt Bremen in einem Tag nicht erreichen und kamen Abends in einen Wald, wo sie übernachten wollten. Der Esel und der Hund legten sich unter einen großen Baum, die Katze und der Hahn machten sich in die Äste, der Hahn aber flog bis in die Spitze, wo es am sichersten für ihn war. Ehe er einschlief, sah er sich noch einmal nach allen vier Winden um, da däuchte ihn er sähe in der Ferne ein Fünkchen brennen und rief seinen Gesellen zu es müßte nicht gar weit ein Haus sein, denn es scheine ein Licht. Sprach der Esel "so müssen wir uns aufmachen und noch hingehen, denn hier ist die Herberge schlecht." Der Hund meinte ein paar Knochen und etwas Fleisch dran, thäten ihm auch gut. Also machten sie sich auf den Weg nach der Gegend, wo das Licht war, und sahen es bald heller schimmern, und es ward immer größer, bis sie vor ein hell erleuchtetes Räuberhaus kamen. Der Esel, als der größte, näherte sich dem Fenster und schaute hinein. "Was siehst du, Grauschimmel?" fragte der Hahn. "Was ich sehe?" antwortete der Esel, "einen gedeckten Tisch mit schönem Essen und Trinken, und Räuber sitzen daran und lassens sich wohl sein." "Das wäre was für uns" sprach der Hahn. "Ja, ja, ach, wären wir da!" sagte der Esel. Da rathschlagten die Thiere wie sie es anfangen müßten, um die Räuber hinaus zu jagen und fanden endlich ein Mittel. Der Esel mußte sich mit den Vorderfüßen auf das Fenster stellen, der Hund auf des Esels Rücken springen, die Katze auf den Hund klettern, und endlich flog der Hahn hinauf, und setzte sich der Katze auf den Kopf. Wie das geschehen war, fiengen sie auf ein Zeichen insgesammt an ihre Musik zu machen: der Esel schrie, der Hund bellte, die Katze miaute und der Hahn krähte; dann stürzten sie durch das Fenster in die Stube hinein daß die Scheiben klirrten. Die Räuber fuhren bei dem entsetzlichen Geschrei in die Höhe, meinten nicht anders als ein Gespenst käme herein und flohen in größter Furcht in den Wald hinaus. Nun setzten sich die vier Gesellen an den Tisch, nahmen mit dem vorlieb, was übrig geblieben war, und aßen als wenn sie vier Wochen hungern sollten.

Wie die vier Spielleute fertig waren, löschten sie das Licht aus und suchten sich eine Schlafstätte, jeder nach seiner Natur und Bequemlichkeit. Der Esel legte sich auf den Mist, der Hund hinter die Thüre, die Katze auf den Herd bei die warme Asche, und der Hahn setzte sich auf den Hahnenbalken: und weil sie müde waren von ihrem langen Weg, schliefen sie auch bald ein. Als Mitternacht vorbei war, und die Räuber von weitem sahen daß kein Licht mehr im Haus brannte, auch alles ruhig schien, sprach der Hauptmann "wir hätten uns doch nicht sollen ins Bockshorn jagen lassen," und hieß einen hingehen und das Haus untersuchen. Der Abgeschickte fand alles still, gieng in die Küche, ein Licht anzuzünden, und weil er die glühenden, feurigen Augen der Katze für lebendige Kohlen ansah, hielt er ein Schwefelhölzchen daran, daß es Feuer fangen sollte. Aber die Katze verstand keinen Spaß, sprang ihm ins Gesicht, spie und kratzte. Da erschrack er gewaltig, lief und wollte zur Hinterthüre hinaus, aber der Hund, der da lag, sprang auf und biß ihn ins Bein: und als er über den Hof an dem Miste vorbei rannte, gab ihm der Esel noch einen tüchtigen Schlag mit dem Hinterfuß; der Hahn aber, der vom Lärmen aus dem Schlaf geweckt und munter geworden war, rief vom Balken herab "kikeriki!" Da lief der Räuber, was er konnte, zu seinem Hauptmann zurück und sprach "ach, in dem Haus sitzt eine gräuliche Hexe, die hat mich angehaucht und mit ihren langen Fingern mir das Gesicht zerkratzt: und vor der Thüre steht ein Mann mit einem Messer, der hat mich ins Bein gestochen: und auf dem Hof liegt ein schwarzes Ungethüm, das hat mit einer Holzkeule auf mich losgeschlagen: und oben auf dem Dache, da sitzt der Richter, der rief bringt mir den Schelm her. Da machte ich daß ich fortkam." Von nun an getrauten sich die Räuber nicht weiter in das Haus, den vier Bremer Musikanten gefiels aber so wohl darin, daß sie nicht wieder heraus wollten. Und der das zuletzt erzählt hat, dem ist der Mund noch warm.
```

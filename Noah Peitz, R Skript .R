A1) 
colnames(df)   # Listet alle Spaltennamen auf – die brauche ich für die Befehle später!

A2)
# ---- [Alle Textspalten umwandeln] ----
df[] <- lapply(df, function(x) {
  if (is.character(x)) as.factor(x) else x
})
str(df)   # Kontrolle: "chr" sollten jetzt "Factor" sein.


# ---- [Duplikate] ----
sum(duplicated(df))             # Wie viele Zeilen sind komplett doppelt?
# df <- df[!duplicated(df), ]   # Doppelte Zeilen entfernen (Raute entfernen zum Aktivieren)


colSums(is.na(df))   # Wie viele leere Zellen pro Spalte?

A3)

lapply(df[, sapply(df, is.factor)], levels)

sapply(df, function(x) length(unique(x)))


A4) 
mean(df$Engine.Size.L., na.rm = TRUE)


mean(df$Fuel.Consumption.Comb..L.100.km., na.rm = TRUE)



mean(df$CO2.Emissions.g.km., na.rm = TRUE)


median(df$Engine.Size.L., na.rm = TRUE) 


median(df$Fuel.Consumption.Comb..L.100.km., na.rm = TRUE) 


median(df$CO2.Emissions.g.km., na.rm = TRUE) 



var(df$Engine.Size.L., na.rm = TRUE)  

var(df$Fuel.Consumption.Comb..L.100.km., na.rm = TRUE)

var(df$CO2.Emissions.g.km., na.rm = TRUE)


sd(df$Engine.Size.L., na.rm = TRUE)   
sd(df$Fuel.Consumption.Comb..L.100.km., na.rm = TRUE)  

sd(df$CO2.Emissions.g.km., na.rm = TRUE) 


A6)
tab_g <- table(df$Vehicle.Class)   

barplot(tab_g,
        main = "Vehicle Class Verteilung",
        xlab = "Vehicle Class", ylab = "Anzahl",
        col  = c("lightblue", "lightpink"))

barplot(prop.table(tab_g) * 100,
        main = "Vehicle Class Verteilung (%)",
        ylab = "Prozent", col = "steelblue")


summary(df)   



A7)

hist(df$CO2.Emissions.g.km., probability = TRUE,
     main = "CO2 Emissions + Glockenkurve",
     xlab = "", col = "lightgray", breaks = 20)
curve(dnorm(x, mean = mean(df$CO2.Emissions.g.km., na.rm=TRUE),
            sd   = sd(df$CO2.Emissions.g.km.,   na.rm=TRUE)),
      add = TRUE, col = "red", lwd = 2)




hist(df$Fuel.Consumption.City..L.100.km., probability = TRUE,
     main = "Fuel Consumption City + Glockenkurve",
     xlab = "", col = "lightgray", breaks = 20)
curve(dnorm(x, mean = mean(df$Fuel.Consumption.City..L.100.km., na.rm=TRUE),
            sd   = sd(df$Fuel.Consumption.City..L.100.km.,   na.rm=TRUE)),
      add = TRUE, col = "red", lwd = 2)


A8)


boxplot(CO2.Emissions.g.km. ~ Fuel.Type, data = df,
            main = "Emission nach Fueltype",
            xlab = "Fuel", ylab = "Emission",
            col  = c("lightblue", "lightpink"))


A9) 
cor.test(df$Fuel.Consumption.Comb..L.100.km., df$CO2.Emissions.g.km., method = "pearson")

A10)


A11)
Für Wahrscheinlichkeit habe ich einen  Taschenrechner genutzt, anstelle von R

A12) binomial

A13)????
  
A14)



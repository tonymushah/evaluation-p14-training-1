-- Your SQL goes here
CREATE VIEW V_RAM_REF_LAPTOP AS
SELECT REF_LAPTOP,
	SUM(PUISSANCE) AS PUISSANCE,
	AVG(FREQUENCE) AS FREQUENCE
FROM RAM_REF_LAPTOP
GROUP BY REF_LAPTOP;
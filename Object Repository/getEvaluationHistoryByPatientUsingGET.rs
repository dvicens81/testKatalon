<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>getEvaluationHistoryByPatientUsingGET</name>
   <tag></tag>
   <elementGuidId>b58eabd8-31e6-4019-9537-4e6c0301cc69</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-type</name>
      <type>Main</type>
      <value>*/*</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>https://localhost:9098/clevehr-services/api/v1/patients/${patientId}${/reasonEncounterHistory}?limit=&amp;skip=</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>1b8cc8a1-7fd4-4fd2-a5a2-96fdf93ff67b</id>
      <masked>false</masked>
      <name>patientId</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>